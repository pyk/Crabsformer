// Copyright (c) 2019, Bayu Aldi Yansyah <bayualdiyansyah@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num::{FromPrimitive, Num};

/// One-dimensional vector parameters
pub struct OneDimensionalVectorParams<T>
where
    T: Num + FromPrimitive + Copy,
{
    /// The shape of the vector.
    ///
    /// The shape of the vector can be specified using
    /// [`with_shape()`].
    ///
    /// [`with_shape()`]: #method.with_shape
    pub shape: Option<[usize; 1]>,

    /// Default value for each element of the vector.
    ///
    /// The default value for each element of the vector
    /// can be specified using [`full_of()`], [`zeros()`]
    /// and [`ones()`].
    ///
    /// [`full_of()`]: #method.full_of
    /// [`zeros()`]: #method.zeros
    /// [`ones()`]: #method.ones
    pub default_value: Option<T>,
}

impl<T> OneDimensionalVectorParams<T>
where
    T: Num + FromPrimitive + Copy,
{
    /// Set the shape of the vector.
    ///
    /// # Examples
    /// ````
    /// # use crabsformer::prelude::*;
    /// let v: Vec<i32> = Vec::one_dim()
    ///     .with_shape([3])
    ///     .zeros()
    ///     .generate();
    /// assert_eq!(v, [0, 0, 0]);
    /// ````
    pub fn with_shape(
        &self,
        shape: [usize; 1],
    ) -> OneDimensionalVectorParams<T> {
        OneDimensionalVectorParams {
            shape: Some(shape),
            default_value: None,
        }
    }

    /// Set the value of all elements of the vector
    /// with `value`.
    ///
    /// # Examples
    /// ````
    /// # use crabsformer::prelude::*;
    /// let v: Vec<i32> = Vec::one_dim()
    ///     .with_shape([3])
    ///     .full_of(5)
    ///     .generate();
    /// assert_eq!(v, [5, 5, 5]);
    /// ````
    pub fn full_of(&self, value: T) -> OneDimensionalVectorParams<T> {
        OneDimensionalVectorParams {
            shape: self.shape,
            default_value: Some(value),
        }
    }

    /// Set the value of all elements of the vector
    /// with zeros.
    ///
    /// # Examples
    /// ````
    /// # use crabsformer::prelude::*;
    /// let v: Vec<i32> = Vec::one_dim()
    ///     .with_shape([3])
    ///     .zeros()
    ///     .generate();
    /// assert_eq!(v, [0, 0, 0]);
    /// ````
    pub fn zeros(&self) -> OneDimensionalVectorParams<T> {
        self.full_of(T::from_i32(0).unwrap())
    }

    /// Set the value of all elements of the vector
    /// with ones.
    ///
    /// # Examples
    /// ````
    /// # use crabsformer::prelude::*;
    /// let v: Vec<i32> = Vec::one_dim()
    ///     .with_shape([3])
    ///     .ones()
    ///     .generate();
    /// assert_eq!(v, [1, 1, 1]);
    /// ````
    pub fn ones(&self) -> OneDimensionalVectorParams<T> {
        self.full_of(T::from_i32(1).unwrap())
    }

    /// Generate the one-dimensional vector based on the
    /// configured parameters.
    ///
    /// # Panics
    /// Panics if the `shape` is not specified.
    ///
    /// # Examples
    /// ````
    /// # use crabsformer::prelude::*;
    /// let v: Vec<i32> = Vec::one_dim()
    ///     .with_shape([3])
    ///     .ones()
    ///     .generate();
    /// assert_eq!(v, [1, 1, 1]);
    /// ````
    pub fn generate(&self) -> Vec<T> {
        // If the shape is not specified then panic
        if self.shape.is_none() {
            panic!("Vector's shape should be specified");
        }

        // Get the size of the vector
        let shape = self.shape.unwrap();
        let size = shape[0];
        let mut output = Vec::with_capacity(size);
        // If default value is provided then populate the vector
        // with the default value
        if self.default_value.is_some() {
            let default_value = self.default_value.unwrap();
            for _ in 0..size {
                output.push(default_value);
            }
        }
        output
    }
}

/// One-dimensional vector builder
pub trait OneDimensional<T>
where
    T: Num + FromPrimitive + Copy,
{
    /// One-dimensional vector builder. It returns [`OneDimensionalVectorParams`]
    /// with default value:
    ///
    /// ```ignore
    /// OneDimensionalVectorParams{
    ///     shape: None,
    ///     default_value: None,
    /// }
    /// ```
    ///
    /// The parameter can be configured using
    /// the following methods: [`with_shape()`], [`full_of()`],
    /// [`ones()`] and [`zeros()`]. After the one-dimensional vector
    /// parameters are configured, use [`generate()`] to generate
    /// the one-dimensional vector.
    ///
    /// [`OneDimensionalVectorParams`]: struct.OneDimensionalVectorParams.html
    /// [`with_shape()`]: struct.OneDimensionalVectorParams.html#method.with_shape
    /// [`full_of()`]: struct.OneDimensionalVectorParams.html#method.full_of
    /// [`zeros()`]: struct.OneDimensionalVectorParams.html#method.zeros
    /// [`ones()`]: struct.OneDimensionalVectorParams.html#method.ones
    /// [`generate()`]: struct.OneDimensionalVectorParams.html#method.generate
    ///
    /// # Examples
    /// ```
    /// # use crabsformer::prelude::*;
    /// // Generate a one-dimensional vector with shape [5]
    /// // filled with zeros; f64 can be changed into any
    /// // numeric data types.
    /// let bias: Vec<f64> = Vec::one_dim()
    ///     .with_shape([5])
    ///     .zeros()
    ///     .generate();
    ///
    /// assert_eq!(bias, [0.0, 0.0, 0.0, 0.0, 0.0]);
    /// ```
    fn one_dim() -> OneDimensionalVectorParams<T>;
}

impl<T> OneDimensional<T> for Vec<T>
where
    T: Num + FromPrimitive + Copy,
{
    fn one_dim() -> OneDimensionalVectorParams<T> {
        OneDimensionalVectorParams {
            shape: None,
            default_value: None,
        }
    }
}
