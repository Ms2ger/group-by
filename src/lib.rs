// Copyright 2015 Ms2ger. See the COPYRIGHT file at the top-level directory of
// this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::hash::Hash;

/// Group the iterator by the result of `f`. Similar to Ruby's `group_by`.
pub fn group_by<T, K, F>(x: T, f: F) -> HashMap<K, Vec<T::Item>>
    where T: IntoIterator,
          F: Fn(&T::Item) -> K,
          K: Eq + Hash
{
    let mut map = HashMap::new();
    for item in x {
        let hash = f(&item);
        map.entry(hash).or_insert(vec![]).push(item);
    }
    map
}

#[test]
fn it_works() {
    let map = group_by(1..7, |i| i%3);
    let expected = vec![
        (0, vec![3, 6]),
        (1, vec![1, 4]),
        (2, vec![2, 5]),
    ].into_iter().collect();
    assert_eq!(map, expected);
}
