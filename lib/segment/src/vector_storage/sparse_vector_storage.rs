use std::ops::Range;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use bitvec::slice::BitSlice;
use common::types::PointOffsetType;

use super::vector_storage_base::VectorStorage;
use super::VectorStorageEnum;
use crate::common::operation_error::OperationResult;
use crate::common::Flusher;
use crate::data_types::vectors::VectorOrSparseRef;
use crate::types::Distance;

pub struct SparseVectorStorage {}

impl VectorStorage for SparseVectorStorage {
    fn vector_dim(&self) -> usize {
        todo!()
    }

    fn distance(&self) -> Distance {
        todo!()
    }

    fn is_on_disk(&self) -> bool {
        todo!()
    }

    fn total_vector_count(&self) -> usize {
        todo!()
    }

    fn get_vector(&self, _key: PointOffsetType) -> VectorOrSparseRef {
        todo!()
    }

    fn insert_vector(
        &mut self,
        _key: PointOffsetType,
        _vector: VectorOrSparseRef,
    ) -> OperationResult<()> {
        todo!()
    }

    fn update_from(
        &mut self,
        _other: &VectorStorageEnum,
        _other_ids: &mut dyn Iterator<Item = PointOffsetType>,
        _stopped: &AtomicBool,
    ) -> OperationResult<Range<PointOffsetType>> {
        todo!()
    }

    fn flusher(&self) -> Flusher {
        todo!()
    }

    fn files(&self) -> Vec<PathBuf> {
        todo!()
    }

    fn delete_vector(&mut self, _key: PointOffsetType) -> OperationResult<bool> {
        todo!()
    }

    fn is_deleted_vector(&self, _key: PointOffsetType) -> bool {
        todo!()
    }

    fn deleted_vector_count(&self) -> usize {
        todo!()
    }

    fn deleted_vector_bitslice(&self) -> &BitSlice {
        todo!()
    }
}
