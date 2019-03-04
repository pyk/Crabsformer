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

//! Gulali is an easy-to-use fundamental library for scientific computing with
//! Rust, highly inspired by [NumPy].
//!
//! [NumPy]: http://www.numpy.org/
//!
//!
//! ## Usage
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! gulali = "2019.3.6"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate gulali;
//!
//! // Import all required traits
//! use gulali::prelude::*;
//! ```
//!
//! To get started using Gulali, read the quickstart tutorial below.
//!
//! ## Quickstart Tutorial
//!
//! ### Prerequisites
//! Before reading this quick tutorial you should know a bit of Rust.
//! If you would like to refresh your memory, take a look at the
//! [Rust book].
//!
//! [Rust book]: https://doc.rust-lang.org/book/
//!
//! ### The Basics
//! Gulali's main data structure is the homogeneous multidimensional [vector].
//! It is a table of elements (usually numbers), all of the same type,
//! indexed by a tuple of positive integers.
//!
//! [vector]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//!
//! For example, the coordinates of a point in 3D space `[1, 2, 1]` has
//! one dimension. That dimension has 3 elements in it, so we say
//! it has a length of 3. In the example pictured below,
//! the vector has 2 dimensions. The first dimension has a length of 2,
//! the second dimension has a length of 3.
//!
//! ```ignore
//! [
//!     [ 1., 0., 0.],
//!     [ 0., 1., 2.]
//! ]
//! ```
//!
//! Gulali uses Rust's [vector] standard data structure extensively.
//! Gulali don't reinvent yet-another data strucutre to keep things
//! simple and easy to use. Gulali add attributes to Rust's [vector]
//! like the following:
//!
//! - [`dim()`]: the number of dimensions of the vector.
//! - [`shape()`]: This is a list of integers indicating the
//!   size of the vector in each dimension.
//!   For a matrix with `n` rows and `m` columns, shape will be `[n,m]`.
//!   The length of the shape is therefore the number of
//!   dimensions, `dim()`.
//! - [`size()`]: the total number of elements of the vector.
//!   This is equal to the product of the elements of shape.
//!
//! [slice]: https://doc.rust-lang.org/rust-by-example/primitives/array.html
//! [`dim()`]: attributes/trait.Dimension.html#tymethod.dim
//! [`shape()`]: attributes/trait.Shape.html#tymethod.shape
//! [`size()`]: attributes/trait.Size.html#tymethod.size
//!
//! ### An Example
//! ```rust
//! # use gulali::prelude::*;
//! // Create two-dimensional vector with shape [3, 3]
//! // filled with zeros
//! let matrix: Vec<Vec<i32>> = Vec::two_dim(3, 3).zeros();
//!
//! assert_eq!(matrix.dim(), 2);
//! assert_eq!(matrix.shape(), [3, 3]);
//! assert_eq!(matrix.size(), 9);
//! ```
//!
//! ### Vector Creation
//! There are several ways to create vectors.
//!
//! For example, you can create a vector using [`vec!`][vec!] macro
//! like the following:
//!
//! ```rust
//! # use gulali::prelude::*;
//! let a = vec![1, 2, 3];
//! assert_eq!(a.dim(), 1);
//! assert_eq!(a.shape(), [3]);
//! assert_eq!(a.size(), 3);
//!
//! let b = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![8, 9, 10],
//! ];
//! assert_eq!(b.dim(), 2);
//! assert_eq!(b.shape(), [3, 3]);
//! assert_eq!(b.size(), 9);
//! ```
//!
//! [vec!]: https://doc.rust-lang.org/std/macro.vec.html
//!
//! The type of the resulting vector is deduced from the type
//! of the elements in the macro.
//!
//! Often, the elements of a vector are originally unknown, but
//! its shape is known. Hence, Gulali offers several functions to
//! create vectors with initial placeholder content. These minimize
//! the necessity of growing vectors, an expensive operation.
//!
//! The function [`zeros()`] creates a vector full of zeros,
//! and the function [`ones()`] creates a vector full of ones.
//!
//! [`zeros()`]: builders/zeros/trait.Zero.html#tymethod.zeros
//! [`ones()`]: builders/ones/trait.One.html#tymethod.ones
//!
// TODO: ADD `rand()`, keyg
//!
//! ```rust
//! # use gulali::prelude::*;
//! // Create two-dimensional vector with shape [3, 3]
//! // filled with zeros
//! let a: Vec<Vec<i32>> = Vec::two_dim(3, 3).zeros();
//! assert_eq!(a, [[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
//!
//! // Create three-dimensional vector with shape [1, 1, 3]
//! // filled with ones
//! let b: Vec<Vec<Vec<f64>>> = Vec::three_dim(1, 1, 3).ones();
//! assert_eq!(b, [[[1.0, 1.0, 1.0]]]);
//! ```
//!
//! ## Getting help
//! Feel free to start discussion at [GitHub issues].
//!
//! [Github issues]: https://github.com/pyk/gulali/issues/new/choose
//!
//! ## License
//! Gulali is licensed under the [Apache-2.0] license.
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in Gulali by you, as defined in the Apache-2.0
//! license, shall be licensed as above, without
//! any additional terms or conditions.
//!
//! [Apache-2.0]: https://github.com/pyk/gulali/blob/master/LICENSE
//!

pub mod builders;

pub mod attributes;

pub mod prelude;
