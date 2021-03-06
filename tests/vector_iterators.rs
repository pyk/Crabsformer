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

use crabsformer::prelude::*;

#[test]
fn test_elements() {
    let x = vector![1, 2, 3];
    let mut elements = x.elements();

    assert_eq!(elements.next(), Some(&1));
    assert_eq!(elements.next(), Some(&2));
    assert_eq!(elements.next(), Some(&3));
    assert_eq!(elements.next(), None);
}
