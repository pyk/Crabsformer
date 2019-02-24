extern crate num;

use num::Num;

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

trait OneDimensional<T> where T: Num + Copy {
    fn one_dim(size: usize) -> Self;
}

impl<T> OneDimensional<T> for Vec<T> where T: Num + Copy {
    fn one_dim(size: usize) -> Vec<T> {
        Vec::with_capacity(size)
    }
}

trait TwoDimensional<T> where T: Num + Copy {
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

trait ThreeDimensional<T> where T: Num + Copy {
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

trait FourDimensional<T> where T: Num + Copy {
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


trait Fill<T> where T: Num + Copy {
    fn fill(&mut self, value: T) -> Self;
}

impl<T> Fill<T> for Vec<T> where T: Num + Copy {
    fn fill(&mut self, value: T) -> Vec<T> {
        for _ in 0..self.capacity() {
            self.push(value);
        }
        self.to_vec()
    }
}

impl<T> Fill<T> for Vec<Vec<T>> where T: Num + Copy {
    fn fill(&mut self, value: T) -> Vec<Vec<T>> {
        for i in 0..self.capacity() {
            for _ in 0..self[i].capacity() {
                self[i].push(value);
            }
        }
        self.to_vec()
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
        let arr: Vec<f64> = Vec::one_dim(5);
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
    fn test_fill() {
        // 1D array
        let arr1: Vec<i32> = Vec::one_dim(5).fill(0);
        assert_eq!(arr1, vec![0, 0, 0, 0, 0]);

        let arr2: Vec<Vec<f64>> = Vec::two_dim(2, 2).fill(5.0);
        assert_eq!(arr2, vec![
            vec![5.0, 5.0],
            vec![5.0, 5.0],
        ]);
    }
}
