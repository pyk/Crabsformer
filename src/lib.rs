pub enum DataType {
    Int64,
    Float64
}

// Array is a homogeneous multidimensional array.
pub struct Array<T> {
    // the number of axes (dimensions) of the array.
    ndim: usize,
    // the dimensions of the array. This is a tuple of integers
    // indicating the size of the array in each dimension.
    // For a matrix with n rows and m columns, shape will be (n,m).
    // The length of the shape tuple is therefore
    // the number of axes, ndim.
    shape: Vec<usize>,
    // the total number of elements of the array. This is equal to
    // the product of the elements of shape.
    size: usize,
    // an Enum describing the type of the elements in the array.
    // One can create or specify dtype’s using enum DataType.
    // dtype: DataType,

    // the size in bytes of each element of the array.
    // For example, an array of elements of type float64
    // has itemsize 8 (=64/8),
    // while one of type complex32 has itemsize 4 (=32/8).
    // It is equivalent to ndarray.dtype.itemsize.
    // item_size: usize,
    // the buffer containing the actual elements of the array.
    // Normally, we won’t need to use this attribute because we will
    // access the elements in an array using indexing facilities.
    data: Vec<T>,
}

impl<T> Array<T> {
    // Initialize Array with specified shape
    pub fn with_shape(shape: Vec<usize>) -> Array<T> {
        let ndim = shape.len();
        let size = shape.iter().product();
        return Array {
            ndim,
            shape,
            size,
            data: Vec::new(),
        };
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_with_shape() {
        let arr: Array<i64> = Array::with_shape(vec![2, 3]);
        assert_eq!(arr.ndim, 2);
        assert_eq!(arr.shape, vec![2, 3]);
        assert_eq!(arr.size, 6);
    }

    // #[test]
    // fn test_zeros() {

    // }
}
