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

pub trait Array1D {
    // Array creation routines
    fn zeros(shape: usize) -> Self;
}

impl Array1D for Vec<i64> {
    fn zeros(shape: usize) -> Vec<i64> {
        let mut array = Vec::with_capacity(shape);
        for _ in 0..shape {
            array.push(0);
        }
        array
    }
}

impl Array1D for Vec<i32> {
    fn zeros(shape: usize) -> Vec<i32> {
        let mut array = Vec::with_capacity(shape);
        for _ in 0..shape {
            array.push(0);
        }
        array
    }
}

impl Array1D for Vec<f64> {
    fn zeros(shape: usize) -> Vec<f64> {
        let mut array = Vec::with_capacity(shape);
        for _ in 0..shape {
            array.push(0.0);
        }
        array
    }
}

impl Array1D for Vec<f32> {
    fn zeros(shape: usize) -> Vec<f32> {
        let mut array = Vec::with_capacity(shape);
        for _ in 0..shape {
            array.push(0.0);
        }
        array
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
    fn test_zeros() {
        // 1D array
        let array_1d_i64: Vec<i64> = Vec::zeros(5);
        assert_eq!(array_1d_i64, vec![0, 0, 0, 0, 0]);

        let array_1d_i32: Vec<i32> = Vec::zeros(5);
        assert_eq!(array_1d_i32, vec![0, 0, 0, 0, 0]);

        let array_1d_f64: Vec<f64> = Vec::zeros(5);
        assert_eq!(array_1d_f64, vec![0.0, 0.0, 0.0, 0.0, 0.0]);

        let array_1d_f32: Vec<f32> = Vec::zeros(5);
        assert_eq!(array_1d_f32, vec![0.0, 0.0, 0.0, 0.0, 0.0]);

    }
}
