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

trait Size<T> where T: Num + Copy {
    fn with_size(size: usize) -> Vec<T>;
}

impl<T> Size<T> for Vec<T> where T: Num + Copy {
    fn with_size(size: usize) -> Vec<T> {
        Vec::with_capacity(size)
    }
}

trait Shape<T> where T: Num + Copy {
    // TODO: don't use slice.s
    fn with_shape(shape: &[usize]) -> Self;
}

impl<T> Shape<T> for Vec<Vec<T>> where T: Num + Copy {
    fn with_shape(shape: &[usize]) -> Vec<Vec<T>> {
        let mut rows: Vec<Vec<T>> = Vec::with_capacity(shape[0]);
        for _ in 0..shape[0] {
            rows.push(Vec::with_capacity(shape[1]));
        }
        rows
    }
}

impl<T> Shape<T> for Vec<Vec<Vec<T>>> where T: Num + Copy {
    fn with_shape(shape: &[usize]) -> Vec<Vec<Vec<T>>> {
        let mut rows: Vec<Vec<Vec<T>>> = Vec::with_capacity(shape[0]);
        for _ in 0..shape[0] {
            let mut cols: Vec<Vec<T>> = Vec::with_capacity(shape[1]);
            for _ in 0..shape[1] {
                cols.push(Vec::with_capacity(shape[2]));
            }
            rows.push(cols);
        }
        rows
    }
}

// trait Shape4D<T> where T: Num + Copy {
//     fn with_shape(shape: [usize; 4]) -> Vec<Vec<Vec<Vec<T>>>>;
// }


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
    fn test_with_size() {
        let arr: Vec<i32> = Vec::with_size(5);
        assert_eq!(arr.capacity(), 5);

        let arr: Vec<i64> = Vec::with_size(5);
        assert_eq!(arr.capacity(), 5);
    }


    #[test]
    fn test_with_shape() {
        let arr1: Vec<Vec<i32>> = Vec::with_shape(&[5, 5]);
        assert_eq!(arr1.capacity(), 5);
        for i in 0..5 {
            assert_eq!(arr1[i].capacity(), 5);
        }

        let arr2: Vec<Vec<u64>> = Vec::with_shape(&[5, 10]);
        assert_eq!(arr2.capacity(), 5);
        for i in 0..5 {
            assert_eq!(arr2[i].capacity(), 10);
        }
    }

    #[test]
    fn test_fill() {
        // 1D array
        let arr1: Vec<i32> = Vec::with_size(5).fill(0);
        assert_eq!(arr1, vec![0, 0, 0, 0, 0]);

        let arr2: Vec<Vec<f64>> = Vec::with_shape(&[2, 2]).fill(5.0);
        assert_eq!(arr2, vec![
            vec![5.0, 5.0],
            vec![5.0, 5.0],
        ]);
    }
}
