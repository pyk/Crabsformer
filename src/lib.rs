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


trait Full<T> where T: Num + Copy {
    // Return a new array of given shape and type, filled with `value`.
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

trait Zero {
    fn zeros(&mut self) -> Self;
}

impl Zero for Vec<i32> {
    fn zeros(&mut self) -> Vec<i32> {
        self.full(0)
    }
}

impl Zero for Vec<i64> {
    fn zeros(&mut self) -> Vec<i64> {
        self.full(0)
    }
}


impl Zero for Vec<f32> {
    fn zeros(&mut self) -> Vec<f32> {
        self.full(0.0)
    }
}

impl Zero for Vec<f64> {
    fn zeros(&mut self) -> Vec<f64> {
        self.full(0.0)
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
        let arr2: Vec<Vec<f64>> = Vec::two_dim(2, 2).full(5.0);
        assert_eq!(arr2, vec![
            vec![5.0, 5.0],
            vec![5.0, 5.0],
        ]);
    }

    #[test]
    fn test_full_three_dim() {
        let arr3: Vec<Vec<Vec<f64>>> = Vec::three_dim(2, 2, 2).full(5.0);
        assert_eq!(arr3,
            vec![
                vec![
                    vec![5.0, 5.0],
                    vec![5.0, 5.0],
                ],
                vec![
                    vec![5.0, 5.0],
                    vec![5.0, 5.0],
                ],
            ]
        );
    }

    #[test]
    fn test_full_four_dim() {
        let arr4: Vec<Vec<Vec<Vec<f64>>>> = Vec::four_dim(2, 2, 2, 2).full(5.0);
        assert_eq!(arr4,
            vec![
                vec![
                    vec![
                        vec![5.0, 5.0],
                        vec![5.0, 5.0],
                    ],
                    vec![
                        vec![5.0, 5.0],
                        vec![5.0, 5.0],
                    ],
                ],
                vec![
                    vec![
                        vec![5.0, 5.0],
                        vec![5.0, 5.0],
                    ],
                    vec![
                        vec![5.0, 5.0],
                        vec![5.0, 5.0],
                    ],
                ],
            ]
        );
    }
}
