pub struct Array2Df64 {
    shape: [usize; 2],
    data: Vec<Vec<f64>>,
}

impl Array2Df64 {
    pub fn with_shape(shape: [usize; 2]) -> Array2Df64 {
        // Initialize 2D array with specified shape
        return Array2Df64 {
            shape,
            data: Vec::with_capacity(shape[0]),
        };
    }

    pub fn zeros(&mut self) -> Array2Df64 {
        for _ in 0..self.shape[0] {
            let mut column = Vec::with_capacity(self.shape[1]);
            for _ in 0..self.shape[1] {
                column.push(0.0);
            }
            self.data.push(column);
        }
        self
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
    fn test_zeros() {
        let a = Array2D::with_shape([2, 2]).zeros();
        assert_eq!(a.shape, [2, 2]);
    }
}
