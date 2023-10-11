use std::marker::PhantomData;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use bitvec::prelude::{BitSlice, BitVec};
use common::types::PointOffsetType;
use rand::Rng;

use crate::common::operation_error::OperationResult;
use crate::common::Flusher;
use crate::data_types::vectors::{
    VectorElementType, VectorOrSparse, VectorOrSparseRef, VectorType,
};
use crate::payload_storage::FilterContext;
use crate::spaces::metric::Metric;
use crate::types::Distance;
use crate::vector_storage::chunked_vectors::ChunkedVectors;
use crate::vector_storage::{
    raw_scorer_impl, RawScorer, VectorStorage, VectorStorageEnum, DEFAULT_STOPPED,
};

pub fn random_vector<R: Rng + ?Sized>(rnd_gen: &mut R, size: usize) -> Vec<VectorElementType> {
    (0..size).map(|_| rnd_gen.gen_range(0.0..1.0)).collect()
}

pub struct FakeFilterContext {}

impl FilterContext for FakeFilterContext {
    fn check(&self, _point_id: PointOffsetType) -> bool {
        true
    }
}

pub struct TestRawScorerProducer<TMetric: Metric> {
    pub vectors: ChunkedVectors<VectorElementType>,
    pub deleted_points: BitVec,
    pub deleted_vectors: BitVec,
    pub metric: PhantomData<TMetric>,
}

impl<TMetric: Metric> VectorStorage for TestRawScorerProducer<TMetric> {
    fn vector_dim(&self) -> usize {
        self.vectors.get(0).len()
    }

    fn distance(&self) -> Distance {
        TMetric::distance()
    }

    fn is_on_disk(&self) -> bool {
        false
    }

    fn total_vector_count(&self) -> usize {
        self.vectors.len()
    }

    fn get_vector(&self, key: PointOffsetType) -> VectorOrSparseRef {
        self.vectors.get(key).into()
    }

    fn insert_vector(
        &mut self,
        key: PointOffsetType,
        vector: VectorOrSparseRef,
    ) -> OperationResult<()> {
        self.vectors.insert(key, vector.try_into()?)?;
        Ok(())
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
        Box::new(|| Ok(()))
    }

    fn files(&self) -> Vec<PathBuf> {
        vec![]
    }

    fn delete_vector(&mut self, key: PointOffsetType) -> OperationResult<bool> {
        Ok(!self.deleted_vectors.replace(key as usize, true))
    }

    fn is_deleted_vector(&self, key: PointOffsetType) -> bool {
        self.deleted_vectors[key as usize]
    }

    fn deleted_vector_count(&self) -> usize {
        self.deleted_vectors.count_ones()
    }

    fn deleted_vector_bitslice(&self) -> &BitSlice {
        &self.deleted_vectors
    }
}

impl<TMetric> TestRawScorerProducer<TMetric>
where
    TMetric: Metric,
{
    pub fn new<R>(dim: usize, num_vectors: usize, rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        let mut vectors = ChunkedVectors::new(dim);
        for _ in 0..num_vectors {
            let rnd_vec = random_vector(rng, dim);
            let rnd_vec = TMetric::preprocess(rnd_vec);
            vectors.push(&rnd_vec).unwrap();
        }
        TestRawScorerProducer::<TMetric> {
            vectors,
            deleted_points: BitVec::repeat(false, num_vectors),
            deleted_vectors: BitVec::repeat(false, num_vectors),
            metric: PhantomData,
        }
    }

    pub fn get_raw_scorer(&self, query: VectorType) -> OperationResult<Box<dyn RawScorer + '_>> {
        let v: VectorOrSparse = TMetric::preprocess(query).into();
        let query = v.into();
        raw_scorer_impl(
            query,
            self,
            self.deleted_vector_bitslice(),
            &DEFAULT_STOPPED,
        )
    }
}
