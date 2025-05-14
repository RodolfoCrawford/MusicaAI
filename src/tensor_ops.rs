use ndarray::{Array, IxDyn}; // [Incremental] for dynamic reshaping
use ndarray::Array;
use num_traits::{One, Zero};
use ort::tensor::PrimitiveTensorElementType;
use ort::value::Tensor;
use std::fmt::Debug;

pub fn zeros_tensor<T: PrimitiveTensorElementType + Debug + Clone + Zero + 'static>(
    shape: &[usize],
) -> Tensor<T> {
    ort::value::Value::from_array(Array::<T, _>::zeros(shape))
        .expect("Could not build zeros tensor")
}

pub fn dupe_zeros_along_first_dim<
    T: PrimitiveTensorElementType + Debug + Zero + Clone + 'static,
>(
    tensor: Tensor<T>,
) -> ort::Result<Tensor<T>> {
    let (shape, data) = tensor.try_extract_raw_tensor()?;
    let mut shape = shape.to_vec();
    shape[0] *= 2;
    let data = [data.to_vec(), vec![T::zero(); data.len()]].concat();
    Tensor::from_array((shape, data))
}

pub fn ones_tensor<T: PrimitiveTensorElementType + Debug + Clone + One + 'static>(
    shape: &[usize],
) -> Tensor<T> {
    ort::value::Value::from_array(Array::<T, _>::ones(shape)).expect("Could not build zeros tensor")
}

pub fn full_tensor<T: PrimitiveTensorElementType + Debug + Clone + 'static>(
    shape: &[usize],
    value: T,
) -> Tensor<T> {
    let total_len = shape.iter().product();
    let data = vec![value; total_len];
    Tensor::from_array((shape.to_vec(), data)).expect("Could not build full tensor")
}

// [Incremental] Create an identity matrix (2D) tensor
pub fn identity_tensor<T: PrimitiveTensorElementType + Debug + One + Zero + Clone + 'static>(
    size: usize,
) -> Tensor<T> {
    let mut array = Array::<T, _>::zeros((size, size));
    for i in 0..size {
        array[(i, i)] = T::one();
    }
    Tensor::from_array(array).expect("Could not build identity tensor")
}

// [Incremental] Reshape an existing tensor to a new shape
pub fn reshape_tensor<T: PrimitiveTensorElementType + Debug + Clone + 'static>(
    tensor: Tensor<T>,
    new_shape: &[usize],
) -> ort::Result<Tensor<T>> {
    let (_, data) = tensor.try_extract_raw_tensor()?;
    let total_new: usize = new_shape.iter().product();
    if total_new != data.len() {
        return Err(ort::OrtError::Msg("Reshape size mismatch".into()));
    }
    Tensor::from_array((new_shape.to_vec(), data.to_vec()))
}
