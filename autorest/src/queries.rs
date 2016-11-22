// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

pub type Queries<'r> = BTreeMap<&'r str, &'r str>;

pub trait QueriesFilter {
    fn select(&self) -> Option<Vec<&str>>;
}

impl<'r> QueriesFilter for Queries<'r> {
    fn select(&self) -> Option<Vec<&str>> {
        match self.get("select") {
            Some(ref val) => {
                Some(val.split(',').collect())
            },
            None => None
        }
    }
}