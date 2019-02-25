//! `np` is the fundamental package for scientific computing with Rust.
//! It contains among other things:
//!
//! - a powerful N-dimensional array routines
//! - sophisticated (broadcasting) functions
//! - useful linear algebra, Fourier transform, and random number capabilities
//!
//! Besides its obvious scientific uses, `np` can also be used as an efficient
//! multi-dimensional container of generic data. Arbitrary data-types can be defined.
//! This allows `np` to seamlessly and speedily integrate with a wide variety
//! of databases.
//!
//! `np` is licensed under the [BSD 3-Clause](https://github.com/pyk/np/blob/master/LICENSE)
//! license, enabling reuse with few restrictions.
//!
//! All materials in this documentation are copy from
//! [NumPy documentation](http://www.numpy.org/) with a modification.

extern crate num;

use num::Num;
use std::ops::Index;
use std::slice::SliceIndex;


pub fn zeros(shape: [usize; 2]) -> Vec<Vec<i64>> {
    let mut rows = Vec::with_capacity(shape[0]);
    for _ in 0..shape[0] {
        let mut cols = Vec::with_capacity(shape[1]);
        for _ in 0..shape[1] {
            cols.push(0);
        }
        rows.push(cols);
    }
    rows
}

struct Array<T> {
    ndim: usize,
    shape: Vec<usize>,
    data: Vec<T>
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[index]
        // Index::index(&*self, index)
    }
}



/// One-dimensional vectors
pub trait OneDimensional<T> where T: Num + Copy {
    /// Returns new one-dimensional vector with specified `size`.
    // TODO(pyk): Add example
    fn one_dim(size: usize) -> Self;
}

impl<T> OneDimensional<T> for Vec<T> where T: Num + Copy {
    fn one_dim(size: usize) -> Vec<T> {
        Vec::with_capacity(size)
    }
}

/// One-dimensional vectors
pub trait OneDimensionalArray<T> where T: Num + Copy {
    /// Returns new one-dimensional vector with specified `size`.
    // TODO(pyk): Add example
    fn one_dim_array(size: usize) -> Self;
}

impl<T> OneDimensionalArray<T> for Array<T> where T: Num + Copy {
    fn one_dim_array(size: usize) -> Array<T> {
        Array {
            ndim: 1,
            shape: vec![size],
            data: Vec::with_capacity(size)
        }
    }
}


/// Two-dimensional vectors
pub trait TwoDimensionalArray<T> where T: Num + Copy {
    /// Returns new two-dimensional vector with specified
    /// number of rows and number of columns.
    ///
    // TODO(pyk): Add example
    fn two_dim_array(nrows: usize, ncols: usize) -> Self;
}

impl<T> TwoDimensionalArray<T> for Array<T> where T: Num + Copy {
    fn two_dim_array(nrows: usize, ncols: usize) -> Array<T> {
        Array {
            ndim: 2,
            shape: vec![nrows, ncols],
            data: Vec::new()
        }
    }
}


/// Two-dimensional vectors
pub trait TwoDimensional<T> where T: Num + Copy {
    /// Returns new two-dimensional vector with specified
    /// number of rows and number of columns.
    ///
    // TODO(pyk): Add example
    fn two_dim(nrows: usize, ncols: usize) -> Self;
}

impl<T> TwoDimensional<T> for Vec<Vec<T>> where T: Num + Copy {
    fn two_dim(nrows: usize, ncols: usize) -> Vec<Vec<T>> {
        let mut array2d: Vec<Vec<T>> = Vec::with_capacity(nrows);
        for _ in 0..nrows {
            array2d.push(Vec::with_capacity(ncols));
        }
        array2d
    }
}

/// Three-dimensional vectors
pub trait ThreeDimensional<T> where T: Num + Copy {
    /// Returns new three-dimensional vector with specified shape.
    ///
    /// # Arguments
    ///
    /// * `n1` - A number of elements in axis 1.
    /// * `n2` - A number of elements in axis 2.
    /// * `n3` - A number of elements in axis 3.
    ///
    // TODO(pyk): Add example
    fn three_dim(n1: usize, n2: usize, n3: usize) -> Self;
}

impl<T> ThreeDimensional<T> for Vec<Vec<Vec<T>>> where T: Num + Copy {
    fn three_dim(n1: usize, n2: usize, n3: usize) -> Vec<Vec<Vec<T>>> {
        let mut array3d: Vec<Vec<Vec<T>>> = Vec::with_capacity(n1);
        for _ in 0..n1 {
            let mut array2d: Vec<Vec<T>> = Vec::with_capacity(n2);
            for _ in 0..n2 {
                array2d.push(Vec::with_capacity(n3));
            }
            array3d.push(array2d);
        }
        array3d
    }
}

/// Four-dimensional vectors
pub trait FourDimensional<T> where T: Num + Copy {
    /// Returns new four-dimensional vector with specified shape.
    ///
    /// # Arguments
    ///
    /// * `n1` - A number of elements in axis 1.
    /// * `n2` - A number of elements in axis 2.
    /// * `n3` - A number of elements in axis 3.
    /// * `n4` - A number of elements in axis 4.
    ///
    // TODO(pyk): Add example
    fn four_dim(n1: usize, n2: usize, n3: usize, n4: usize) -> Self;
}


impl<T> FourDimensional<T> for Vec<Vec<Vec<Vec<T>>>> where T: Num + Copy {
    fn four_dim(n1: usize, n2: usize, n3: usize, n4: usize) -> Vec<Vec<Vec<Vec<T>>>> {
        let mut array4d: Vec<Vec<Vec<Vec<T>>>> = Vec::with_capacity(n1);
        for _ in 0..n1 {
            let mut array3d: Vec<Vec<Vec<T>>> = Vec::with_capacity(n2);
            for _ in 0..n2 {
                let mut array2d: Vec<Vec<T>> = Vec::with_capacity(n3);
                for _ in 0..n3 {
                    array2d.push(Vec::with_capacity(n4));
                }
                array3d.push(array2d);
            }
            array4d.push(array3d)
        }
        array4d
    }
}

/// Fillable vectors
pub trait Full<T> where T: Num + Copy {
    /// Returns a new array of given shape and type, filled with `value`.
    fn full(&mut self, value: T) -> Self;
}

impl<T> Full<T> for Vec<T> where T: Num + Copy {
    // Return a new 1D array of given shape and type, filled with `value`.
    fn full(&mut self, value: T) -> Vec<T> {
        for _ in 0..self.capacity() {
            self.push(value);
        }
        self.to_vec()
    }
}

impl<T> Full<T> for Vec<Vec<T>> where T: Num + Copy {
    // Return a new 2D array of given shape and type, filled with `value`.
    fn full(&mut self, value: T) -> Vec<Vec<T>> {
        for i in 0..self.capacity() {
            for _ in 0..self[i].capacity() {
                self[i].push(value);
            }
        }
        self.to_vec()
    }
}


impl<T> Full<T> for Vec<Vec<Vec<T>>> where T: Num + Copy {
    // Return a new 3D array of given shape and type, filled with `value`.
    fn full(&mut self, value: T) -> Vec<Vec<Vec<T>>> {
        for i in 0..self.capacity() {
            for j in 0..self[i].capacity() {
                for _ in 0..self[i][j].capacity() {
                    self[i][j].push(value);
                }
            }
        }
        self.to_vec()
    }
}


impl<T> Full<T> for Vec<Vec<Vec<Vec<T>>>> where T: Num + Copy {
    // Return a new 4D array of given shape and type, filled with `value`.
    fn full(&mut self, value: T) -> Vec<Vec<Vec<Vec<T>>>> {
        for i in 0..self.capacity() {
            for j in 0..self[i].capacity() {
                for k in 0..self[i][j].capacity() {
                    for _ in 0..self[i][j][k].capacity() {
                        self[i][j][k].push(value);
                    }
                }
            }
        }
        self.to_vec()
    }
}

/// Fillable vectors
pub trait FullArray<T> where T: Num + Copy {
    /// Returns a new array of given shape and type, filled with `value`.
    fn full_array(self, value: T) -> Self;
}

impl<T> FullArray<T> for Array<T> where T: Num + Copy {
    fn full_array(self, value: T) -> Array<T> {
        // Create new Array with new value
        let capacity = self.shape.iter().product();
        let mut new_data = Vec::with_capacity(capacity);
        for _ in 0..new_data.capacity() {
            new_data.push(value);
        }
        Array {
            shape: self.shape,
            ndim: self.ndim,
            data: new_data
        }
    }
}

/// A zero-able vectors
pub trait Zero {
    /// Return a new vector of given data type and shape,
    /// filled with zeros.
    fn zeros(&mut self) -> Self;
}


impl Zero for Vec<u8> {
    fn zeros(&mut self) -> Vec<u8> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<u8>> {
    fn zeros(&mut self) -> Vec<Vec<u8>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<u8>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<u8>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<u8>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<u8>>>> {
        self.full(0)
    }
}


impl Zero for Vec<u16> {
    fn zeros(&mut self) -> Vec<u16> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<u16>> {
    fn zeros(&mut self) -> Vec<Vec<u16>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<u16>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<u16>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<u16>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<u16>>>> {
        self.full(0)
    }
}

impl Zero for Vec<u32> {
    fn zeros(&mut self) -> Vec<u32> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<u32>> {
    fn zeros(&mut self) -> Vec<Vec<u32>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<u32>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<u32>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<u32>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<u32>>>> {
        self.full(0)
    }
}

impl Zero for Vec<u64> {
    fn zeros(&mut self) -> Vec<u64> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<u64>> {
    fn zeros(&mut self) -> Vec<Vec<u64>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<u64>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<u64>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<u64>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<u64>>>> {
        self.full(0)
    }
}

impl Zero for Vec<u128> {
    fn zeros(&mut self) -> Vec<u128> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<u128>> {
    fn zeros(&mut self) -> Vec<Vec<u128>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<u128>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<u128>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<u128>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<u128>>>> {
        self.full(0)
    }
}

impl Zero for Vec<i8> {
    fn zeros(&mut self) -> Vec<i8> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<i8>> {
    fn zeros(&mut self) -> Vec<Vec<i8>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<i8>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<i8>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<i8>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<i8>>>> {
        self.full(0)
    }
}

impl Zero for Vec<i16> {
    fn zeros(&mut self) -> Vec<i16> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<i16>> {
    fn zeros(&mut self) -> Vec<Vec<i16>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<i16>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<i16>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<i16>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<i16>>>> {
        self.full(0)
    }
}

impl Zero for Vec<i32> {
    fn zeros(&mut self) -> Vec<i32> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<i32>> {
    fn zeros(&mut self) -> Vec<Vec<i32>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<i32>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<i32>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<i32>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<i32>>>> {
        self.full(0)
    }
}

impl Zero for Vec<i64> {
    fn zeros(&mut self) -> Vec<i64> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<i64>> {
    fn zeros(&mut self) -> Vec<Vec<i64>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<i64>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<i64>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<i64>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<i64>>>> {
        self.full(0)
    }
}

impl Zero for Vec<i128> {
    fn zeros(&mut self) -> Vec<i128> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<i128>> {
    fn zeros(&mut self) -> Vec<Vec<i128>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<i128>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<i128>>> {
        self.full(0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<i128>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<i128>>>> {
        self.full(0)
    }
}

impl Zero for Vec<f32> {
    fn zeros(&mut self) -> Vec<f32> {
        self.full(0.0)
    }
}

impl Zero for Vec<Vec<f32>> {
    fn zeros(&mut self) -> Vec<Vec<f32>> {
        self.full(0.0)
    }
}

impl Zero for Vec<Vec<Vec<f32>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<f32>>> {
        self.full(0.0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<f32>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<f32>>>> {
        self.full(0.0)
    }
}

impl Zero for Vec<f64> {
    fn zeros(&mut self) -> Vec<f64> {
        self.full(0.0)
    }
}

impl Zero for Vec<Vec<f64>> {
    fn zeros(&mut self) -> Vec<Vec<f64>> {
        self.full(0.0)
    }
}

impl Zero for Vec<Vec<Vec<f64>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<f64>>> {
        self.full(0.0)
    }
}

impl Zero for Vec<Vec<Vec<Vec<f64>>>> {
    fn zeros(&mut self) -> Vec<Vec<Vec<Vec<f64>>>> {
        self.full(0.0)
    }
}


/// A zero-able vectors
pub trait ZeroArray {
    /// Return a new vector of given data type and shape,
    /// filled with zeros.
    fn zeros_array(self) -> Self;
}

impl ZeroArray for Array<i32> {
    fn zeros_array(self) -> Array<i32> {
        self.full_array(0)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_zeros() {
        let arr2d: Vec<Vec<i64>> = zeros([2, 2]);
        assert_eq!(arr2d, vec![
            vec![0, 0],
            vec![0, 0],
        ])
    }

    #[test]
    fn test_one_dim() {
        let arr: Vec<i64> = Vec::one_dim(5);
        assert_eq!(arr.capacity(), 5);
    }

    #[test]
    fn test_two_dim() {
        let arr: Vec<Vec<f64>> = Vec::two_dim(5, 5);
        assert_eq!(arr.capacity(), 5);
        for i in 0..5 {
            assert_eq!(arr[i].capacity(), 5);
        }
    }

    #[test]
    fn test_three_dim() {
        let arr: Vec<Vec<Vec<f64>>> = Vec::three_dim(5, 5, 5);
        assert_eq!(arr.capacity(), 5);
        for i in 0..5 {
            assert_eq!(arr[i].capacity(), 5);
            for j in 0..5 {
                assert_eq!(arr[i][j].capacity(), 5);
            }
        }
    }

    #[test]
    fn test_four_dim() {
        // TODO: Look at numpy array
        let arr: Vec<Vec<Vec<Vec<f64>>>> = Vec::four_dim(5, 5, 5, 5);
        assert_eq!(arr.capacity(), 5);
        for i in 0..5 {
            assert_eq!(arr[i].capacity(), 5);
            for j in 0..5 {
                assert_eq!(arr[i][j].capacity(), 5);
                for k in 0..5 {
                    assert_eq!(arr[i][j][k].capacity(), 5);
                }
            }
        }
    }

    #[test]
    fn test_full_one_dim() {
        // 1D array
        let arr1: Vec<i32> = Vec::one_dim(5).full(0);
        assert_eq!(arr1, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_full_two_dim() {
        let arr2: Vec<Vec<f64>> = Vec::two_dim(1, 2).full(5.0);
        assert_eq!(arr2, vec![vec![5.0, 5.0]]);
    }

    #[test]
    fn test_full_three_dim() {
        let arr3: Vec<Vec<Vec<f64>>> = Vec::three_dim(1, 1, 2).full(5.0);
        assert_eq!(arr3, vec![vec![vec![5.0, 5.0]]]);
    }

    #[test]
    fn test_full_four_dim() {
        let arr4: Vec<Vec<Vec<Vec<f64>>>> = Vec::four_dim(1, 1, 1, 2).full(5.0);
        assert_eq!(arr4, vec![vec![vec![vec![5.0, 5.0]]]]);
    }

    #[test]
    fn test_zeros_u8_one_dim() {
        let arr: Vec<u8> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_u8_two_dim() {
        let arr: Vec<Vec<u8>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_u8_three_dim() {
        let arr: Vec<Vec<Vec<u8>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_u8_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u8>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_u16_one_dim() {
        let arr: Vec<u16> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_u16_two_dim() {
        let arr: Vec<Vec<u16>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_u16_three_dim() {
        let arr: Vec<Vec<Vec<u16>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_u16_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u16>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_u32_one_dim() {
        let arr: Vec<u32> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_u32_two_dim() {
        let arr: Vec<Vec<u32>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_u32_three_dim() {
        let arr: Vec<Vec<Vec<u32>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_u32_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u32>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_u64_one_dim() {
        let arr: Vec<u64> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_u64_two_dim() {
        let arr: Vec<Vec<u64>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_u64_three_dim() {
        let arr: Vec<Vec<Vec<u64>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_u64_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u64>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_u128_one_dim() {
        let arr: Vec<u128> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_u128_two_dim() {
        let arr: Vec<Vec<u128>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_u128_three_dim() {
        let arr: Vec<Vec<Vec<u128>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_u128_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u128>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_i8_one_dim() {
        let arr: Vec<i8> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_i8_two_dim() {
        let arr: Vec<Vec<i8>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_i8_three_dim() {
        let arr: Vec<Vec<Vec<i8>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_i8_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i8>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_i16_one_dim() {
        let arr: Vec<i16> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_i16_two_dim() {
        let arr: Vec<Vec<i16>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_i16_three_dim() {
        let arr: Vec<Vec<Vec<i16>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_i16_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i16>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_i32_one_dim() {
        let arr: Vec<i32> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_i32_two_dim() {
        let arr: Vec<Vec<i32>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_i32_three_dim() {
        let arr: Vec<Vec<Vec<i32>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_i32_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i32>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_i64_one_dim() {
        let arr: Vec<i64> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_i64_two_dim() {
        let arr: Vec<Vec<i64>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_i64_three_dim() {
        let arr: Vec<Vec<Vec<i64>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_i64_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i64>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_i128_one_dim() {
        let arr: Vec<i128> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0, 0]);
    }

    #[test]
    fn test_zeros_i128_two_dim() {
        let arr: Vec<Vec<i128>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0, 0]]);
    }

    #[test]
    fn test_zeros_i128_three_dim() {
        let arr: Vec<Vec<Vec<i128>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0, 0]]]);
    }

    #[test]
    fn test_zeros_i128_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i128>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0, 0]]]]);
    }

    #[test]
    fn test_zeros_f32_one_dim() {
        let arr: Vec<f32> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0.0, 0.0]);
    }

    #[test]
    fn test_zeros_f32_two_dim() {
        let arr: Vec<Vec<f32>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0.0, 0.0]]);
    }

    #[test]
    fn test_zeros_f32_three_dim() {
        let arr: Vec<Vec<Vec<f32>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0.0, 0.0]]]);
    }

    #[test]
    fn test_zeros_f32_four_dim() {
        let arr: Vec<Vec<Vec<Vec<f32>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0.0, 0.0]]]]);
    }

    #[test]
    fn test_zeros_f64_one_dim() {
        let arr: Vec<f64> = Vec::one_dim(2).zeros();
        assert_eq!(arr, vec![0.0, 0.0]);
    }

    #[test]
    fn test_zeros_f64_two_dim() {
        let arr: Vec<Vec<f64>> = Vec::two_dim(1, 2).zeros();
        assert_eq!(arr, vec![vec![0.0, 0.0]]);
    }

    #[test]
    fn test_zeros_f64_three_dim() {
        let arr: Vec<Vec<Vec<f64>>> = Vec::three_dim(1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![0.0, 0.0]]]);
    }

    #[test]
    fn test_zeros_f64_four_dim() {
        let arr: Vec<Vec<Vec<Vec<f64>>>> = Vec::four_dim(1, 1, 1, 2).zeros();
        assert_eq!(arr, vec![vec![vec![vec![0.0, 0.0]]]]);
    }

    #[test]
    fn test_array_one_dim() {
        let arr: Array<i32> = Array::one_dim_array(5);
        assert_eq!(arr.ndim, 1);
        assert_eq!(arr.shape, [5]);
    }

    #[test]
    fn test_array_one_dim_zeros() {
        let arr: Array<i32> = Array::one_dim_array(5).zeros_array();
        assert_eq!(arr.ndim, 1);
        assert_eq!(arr.shape, [5]);
        assert_eq!(arr.data, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_array_one_dim_zeros_index() {
        let arr: Array<i32> = Array::one_dim_array(5).zeros_array();
        for i in 0..5 {
            assert_eq!(arr[i], 0);
        }
    }

    #[test]
    fn test_array_two_dim() {
        let arr: Array<i32> = Array::two_dim_array(5, 5);
        assert_eq!(arr.ndim, 2);
        assert_eq!(arr.shape, [5, 5]);
    }

    #[test]
    fn test_array_two_dim_zeros() {
        let arr: Array<i32> = Array::two_dim_array(2, 2).zeros_array();
        assert_eq!(arr.ndim, 2);
        assert_eq!(arr.shape, [2, 2]);
        assert_eq!(arr.data, [0, 0, 0, 0]);
    }

    #[test]
    fn test_array_two_dim_zeros_index() {
        let arr: Array<i32> = Array::two_dim_array(2, 2).zeros_array();
        for i in 0..2 {
            for j in 0..2 {
                assert_eq!(arr[i][j], 0);
            }
        }
    }
}
