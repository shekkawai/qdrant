pub mod anonymize;
pub mod arc_atomic_ref_cell_iterator;
pub mod cpu;
pub mod error_logging;
pub mod mmap_type;
pub mod operation_error;
pub mod operation_time_statistics;
pub mod rocksdb_buffered_delete_wrapper;
pub mod rocksdb_wrapper;
pub mod utils;
pub mod vector_utils;
pub mod version;

use std::sync::atomic::AtomicBool;

use crate::common::operation_error::{OperationError, OperationResult};
use crate::data_types::named_vectors::NamedVectors;
use crate::data_types::vectors::{QueryVector, VectorOrSparseRef};
use crate::types::{SegmentConfig, VectorDataConfig};

pub type Flusher = Box<dyn FnOnce() -> OperationResult<()> + Send>;

/// Check that the given vector name is part of the segment config.
///
/// Returns an error if incompatible.
pub fn check_vector_name(vector_name: &str, segment_config: &SegmentConfig) -> OperationResult<()> {
    get_vector_config_or_error(vector_name, segment_config)?;
    Ok(())
}

/// Check that the given vector name and elements are compatible with the given segment config.
///
/// Returns an error if incompatible.
pub fn check_vector(
    vector_name: &str,
    query_vector: &QueryVector,
    segment_config: &SegmentConfig,
) -> OperationResult<()> {
    let vector_config = get_vector_config_or_error(vector_name, segment_config)?;
    _check_query_vector(query_vector, vector_config)
}

fn _check_query_vector(
    query_vector: &QueryVector,
    vector_config: &VectorDataConfig,
) -> OperationResult<()> {
    match query_vector {
        QueryVector::Nearest(vector) => check_vector_against_config(vector.into(), vector_config)?,
        QueryVector::Recommend(reco_query) => reco_query
            .iter_all()
            .try_for_each(|vector| check_vector_against_config(vector.into(), vector_config))?,
    }

    Ok(())
}

/// Check that the given vector name and elements are compatible with the given segment config.
///
/// Returns an error if incompatible.
pub fn check_query_vectors(
    vector_name: &str,
    query_vectors: &[&QueryVector],
    segment_config: &SegmentConfig,
) -> OperationResult<()> {
    let vector_config = get_vector_config_or_error(vector_name, segment_config)?;
    query_vectors
        .iter()
        .try_for_each(|qv| _check_query_vector(qv, vector_config))?;
    Ok(())
}

/// Check that the given named vectors are compatible with the given segment config.
///
/// Returns an error if incompatible.
pub fn check_named_vectors(
    vectors: &NamedVectors,
    segment_config: &SegmentConfig,
) -> OperationResult<()> {
    for (vector_name, vector_data) in vectors.iter() {
        check_vector(vector_name, &vector_data.into(), segment_config)?;
    }
    Ok(())
}

/// Get the vector config for the given name, or return a name error.
///
/// Returns an error if incompatible.
fn get_vector_config_or_error<'a>(
    vector_name: &str,
    segment_config: &'a SegmentConfig,
) -> OperationResult<&'a VectorDataConfig> {
    segment_config
        .vector_data
        .get(vector_name)
        .ok_or_else(|| OperationError::VectorNameNotExists {
            received_name: vector_name.into(),
        })
}

/// Check if the given vector data is compatible with the given configuration.
///
/// Returns an error if incompatible.
fn check_vector_against_config(
    vector: VectorOrSparseRef,
    vector_config: &VectorDataConfig,
) -> OperationResult<()> {
    // Check dimensionality
    let dim = vector_config.size;
    if vector.len() != dim {
        return Err(OperationError::WrongVector {
            expected_dim: dim,
            received_dim: vector.len(),
        });
    }
    Ok(())
}

pub fn check_stopped(is_stopped: &AtomicBool) -> OperationResult<()> {
    if is_stopped.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(OperationError::Cancelled {
            description: "Operation is stopped externally".to_string(),
        });
    }
    Ok(())
}

pub const BYTES_IN_KB: usize = 1024;
