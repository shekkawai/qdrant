use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sparse::common::sparse_vector::SparseVector;

use super::named_vectors::NamedVectors;
use crate::common::operation_error::OperationError;
use crate::common::utils::transpose_map_into_named_vector;
use crate::vector_storage::query::reco_query::RecoQuery;

/// Type of vector element.
pub type VectorElementType = f32;

pub const DEFAULT_VECTOR_NAME: &str = "";

/// Type for vector
pub type VectorType = Vec<VectorElementType>;

#[derive(Debug, Clone)]
pub enum VectorOrSparse {
    Vector(VectorType),
    Sparse(SparseVector),
}

#[derive(Clone, Copy)]
pub enum VectorOrSparseRef<'a> {
    Vector(&'a [VectorElementType]),
    Sparse(&'a SparseVector),
}

impl<'a> VectorOrSparseRef<'a> {
    // Cannot use `ToOwned` trait because of `Borrow` implementation for `VectorOrSparse`
    pub fn to_owned(self) -> VectorOrSparse {
        match self {
            VectorOrSparseRef::Vector(v) => VectorOrSparse::Vector(v.to_vec()),
            VectorOrSparseRef::Sparse(v) => VectorOrSparse::Sparse(v.clone()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            VectorOrSparseRef::Vector(v) => v.len(),
            VectorOrSparseRef::Sparse(v) => v.indices.len(),
        }
    }
}

impl<'a> From<&'a VectorType> for VectorOrSparseRef<'a> {
    fn from(v: &'a VectorType) -> Self {
        VectorOrSparseRef::Vector(v)
    }
}

impl From<VectorType> for VectorOrSparse {
    fn from(v: VectorType) -> Self {
        VectorOrSparse::Vector(v)
    }
}

impl<'a> From<&'a [VectorElementType]> for VectorOrSparseRef<'a> {
    fn from(v: &'a [VectorElementType]) -> Self {
        VectorOrSparseRef::Vector(v)
    }
}

impl From<SparseVector> for VectorOrSparse {
    fn from(v: SparseVector) -> Self {
        VectorOrSparse::Sparse(v)
    }
}

impl<'a> From<&'a SparseVector> for VectorOrSparseRef<'a> {
    fn from(v: &'a SparseVector) -> Self {
        VectorOrSparseRef::Sparse(v)
    }
}

impl<'a> Into<VectorOrSparseRef<'a>> for &'a VectorOrSparse {
    fn into(self) -> VectorOrSparseRef<'a> {
        match self {
            VectorOrSparse::Vector(v) => VectorOrSparseRef::Vector(v),
            VectorOrSparse::Sparse(v) => VectorOrSparseRef::Sparse(v),
        }
    }
}

impl<'a> TryInto<&'a [VectorElementType]> for &'a VectorOrSparse {
    type Error = OperationError;

    fn try_into(self) -> Result<&'a [VectorElementType], Self::Error> {
        match self {
            VectorOrSparse::Vector(v) => Ok(v),
            VectorOrSparse::Sparse(_) => Err(OperationError::WrongSparse),
        }
    }
}

impl TryInto<VectorType> for VectorOrSparse {
    type Error = OperationError;

    fn try_into(self) -> Result<VectorType, Self::Error> {
        match self {
            VectorOrSparse::Vector(v) => Ok(v),
            VectorOrSparse::Sparse(_) => Err(OperationError::WrongSparse),
        }
    }
}

impl<'a> TryInto<&'a SparseVector> for &'a VectorOrSparse {
    type Error = OperationError;

    fn try_into(self) -> Result<&'a SparseVector, Self::Error> {
        match self {
            VectorOrSparse::Vector(_) => Err(OperationError::WrongSparse),
            VectorOrSparse::Sparse(v) => Ok(v),
        }
    }
}

impl TryInto<SparseVector> for VectorOrSparse {
    type Error = OperationError;

    fn try_into(self) -> Result<SparseVector, Self::Error> {
        match self {
            VectorOrSparse::Vector(_) => Err(OperationError::WrongSparse),
            VectorOrSparse::Sparse(v) => Ok(v),
        }
    }
}

impl<'a> TryInto<&'a [VectorElementType]> for VectorOrSparseRef<'a> {
    type Error = OperationError;

    fn try_into(self) -> Result<&'a [VectorElementType], Self::Error> {
        match self {
            VectorOrSparseRef::Vector(v) => Ok(v),
            VectorOrSparseRef::Sparse(_) => Err(OperationError::WrongSparse),
        }
    }
}

impl<'a> TryInto<&'a SparseVector> for VectorOrSparseRef<'a> {
    type Error = OperationError;

    fn try_into(self) -> Result<&'a SparseVector, Self::Error> {
        match self {
            VectorOrSparseRef::Vector(_) => Err(OperationError::WrongSparse),
            VectorOrSparseRef::Sparse(v) => Ok(v),
        }
    }
}

pub fn default_vector(vec: Vec<VectorElementType>) -> NamedVectors<'static> {
    NamedVectors::from([(DEFAULT_VECTOR_NAME.to_owned(), vec)])
}

pub fn only_default_vector(vec: &[VectorElementType]) -> NamedVectors {
    NamedVectors::from_ref(DEFAULT_VECTOR_NAME, vec.into())
}

/// Full vector data per point separator with single and multiple vector modes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged, rename_all = "snake_case")]
pub enum VectorStruct {
    Single(VectorType),
    Multi(HashMap<String, VectorType>),
    Sparse(SparseVector),
    MultiSparse(HashMap<String, SparseVector>),
}

impl VectorStruct {
    /// Check if this vector struct is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            VectorStruct::Single(vector) => vector.is_empty(),
            VectorStruct::Multi(vectors) => vectors.values().all(|v| v.is_empty()),
            VectorStruct::Sparse(vector) => vector.indices.is_empty(),
            VectorStruct::MultiSparse(vectors) => vectors.values().all(|v| v.indices.is_empty()),
        }
    }
}

impl From<VectorType> for VectorStruct {
    fn from(v: VectorType) -> Self {
        VectorStruct::Single(v)
    }
}

impl From<SparseVector> for VectorStruct {
    fn from(v: SparseVector) -> Self {
        VectorStruct::Sparse(v)
    }
}

impl From<&[VectorElementType]> for VectorStruct {
    fn from(v: &[VectorElementType]) -> Self {
        VectorStruct::Single(v.to_vec())
    }
}

impl<'a> From<NamedVectors<'a>> for VectorStruct {
    // TODO(ivan): add conversion for sparse vectors
    fn from(v: NamedVectors) -> Self {
        if v.len() == 1 && v.contains_key(DEFAULT_VECTOR_NAME) {
            VectorStruct::Single(v.into_default_vector().unwrap())
        } else {
            VectorStruct::Multi(v.into_owned_map())
        }
    }
}

impl VectorStruct {
    pub fn get(&self, name: &str) -> Option<&VectorType> {
        match self {
            VectorStruct::Single(v) => (name == DEFAULT_VECTOR_NAME).then_some(v),
            VectorStruct::Multi(v) => v.get(name),
            VectorStruct::Sparse(_v) => todo!(), //TODO(ivan)
            VectorStruct::MultiSparse(_v) => todo!(), //TODO(ivan)
        }
    }

    pub fn into_all_vectors(self) -> NamedVectors<'static> {
        match self {
            VectorStruct::Single(v) => default_vector(v),
            VectorStruct::Multi(v) => NamedVectors::from_map(v),
            VectorStruct::Sparse(_v) => todo!(), //NamedVectors::from_sparse(v),
            VectorStruct::MultiSparse(_v) => todo!(), //NamedVectors::from_sparse_map(v),
        }
    }
}

/// Vector data with name
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub struct NamedVector {
    /// Name of vector data
    pub name: String,
    /// Vector data
    pub vector: VectorType,
}

/// Vector data separator for named and unnamed modes
/// Unnamed mode:
///
/// {
///   "vector": [1.0, 2.0, 3.0]
/// }
///
/// or named mode:
///
/// {
///   "vector": {
///     "vector": [1.0, 2.0, 3.0],
///     "name": "image-embeddings"
///   }
/// }
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum NamedVectorStruct {
    Default(VectorType),
    Named(NamedVector),
}

impl From<VectorType> for NamedVectorStruct {
    fn from(v: VectorType) -> Self {
        NamedVectorStruct::Default(v)
    }
}

impl From<NamedVectorStruct> for NamedVector {
    fn from(v: NamedVectorStruct) -> Self {
        match v {
            NamedVectorStruct::Default(v) => NamedVector {
                name: DEFAULT_VECTOR_NAME.to_owned(),
                vector: v,
            },
            NamedVectorStruct::Named(v) => v,
        }
    }
}

impl From<NamedVector> for NamedVectorStruct {
    fn from(v: NamedVector) -> Self {
        NamedVectorStruct::Named(v)
    }
}
pub trait Named {
    fn get_name(&self) -> &str;
}

impl Named for NamedVectorStruct {
    fn get_name(&self) -> &str {
        match self {
            NamedVectorStruct::Default(_) => DEFAULT_VECTOR_NAME,
            NamedVectorStruct::Named(v) => &v.name,
        }
    }
}

impl NamedVectorStruct {
    pub fn get_vector(&self) -> &VectorType {
        match self {
            NamedVectorStruct::Default(v) => v,
            NamedVectorStruct::Named(v) => &v.vector,
        }
    }
    pub fn to_vector(self) -> VectorType {
        match self {
            NamedVectorStruct::Default(v) => v,
            NamedVectorStruct::Named(v) => v.vector,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum BatchVectorStruct {
    Single(Vec<VectorType>),
    Multi(HashMap<String, Vec<VectorType>>),
}

impl From<Vec<VectorType>> for BatchVectorStruct {
    fn from(v: Vec<VectorType>) -> Self {
        BatchVectorStruct::Single(v)
    }
}

impl From<HashMap<String, Vec<VectorType>>> for BatchVectorStruct {
    fn from(v: HashMap<String, Vec<VectorType>>) -> Self {
        if v.len() == 1 && v.contains_key(DEFAULT_VECTOR_NAME) {
            BatchVectorStruct::Single(v.into_iter().next().unwrap().1)
        } else {
            BatchVectorStruct::Multi(v)
        }
    }
}

impl BatchVectorStruct {
    pub fn single(&mut self) -> &mut Vec<VectorType> {
        match self {
            BatchVectorStruct::Single(v) => v,
            BatchVectorStruct::Multi(v) => v.get_mut(DEFAULT_VECTOR_NAME).unwrap(),
        }
    }

    pub fn multi(&mut self) -> &mut HashMap<String, Vec<VectorType>> {
        match self {
            BatchVectorStruct::Single(_) => panic!("BatchVectorStruct is not Single"),
            BatchVectorStruct::Multi(v) => v,
        }
    }

    pub fn into_all_vectors(self, num_records: usize) -> Vec<NamedVectors<'static>> {
        match self {
            BatchVectorStruct::Single(vectors) => vectors.into_iter().map(default_vector).collect(),
            BatchVectorStruct::Multi(named_vectors) => {
                if named_vectors.is_empty() {
                    vec![NamedVectors::default(); num_records]
                } else {
                    transpose_map_into_named_vector(named_vectors)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NamedRecoQuery {
    pub query: RecoQuery<VectorType>,
    pub using: Option<String>,
}

impl Named for NamedRecoQuery {
    fn get_name(&self) -> &str {
        self.using.as_deref().unwrap_or(DEFAULT_VECTOR_NAME)
    }
}

#[derive(Debug, Clone)]
pub enum QueryVector {
    Nearest(VectorOrSparse),
    Recommend(RecoQuery<VectorOrSparse>),
}

impl<'a> From<&'a [VectorElementType]> for QueryVector {
    fn from(vec: &'a [VectorElementType]) -> Self {
        let v: VectorOrSparseRef = vec.into();
        Self::Nearest(v.to_owned())
    }
}

impl From<VectorOrSparse> for QueryVector {
    fn from(vec: VectorOrSparse) -> Self {
        Self::Nearest(vec)
    }
}

impl<'a> From<VectorOrSparseRef<'a>> for QueryVector {
    fn from(vec: VectorOrSparseRef<'a>) -> Self {
        Self::Nearest(vec.to_owned())
    }
}

impl<const N: usize> From<[VectorElementType; N]> for QueryVector {
    fn from(vec: [VectorElementType; N]) -> Self {
        let vec: VectorOrSparseRef = vec.as_slice().into();
        Self::Nearest(vec.to_owned())
    }
}
