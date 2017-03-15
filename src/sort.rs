#![allow(dead_code)]
#![allow(unused_variables)]

extern crate indigo;

use indigo::external_sort::{SortOptions, ExternalSort};
use std::cmp::Ordering;

fn main () {
    let boxed_seq: Box<Iterator<Item=u32>> = Box::new(1..1000);

    use std::fs;

    fs::create_dir_all("/tmp/r-sort/ns-1").unwrap();

    let options = SortOptions {
        directory: "/tmp/r-sort/ns-1/".to_string(),
        chunk_size: 100
    };

    ExternalSort::new(boxed_seq, options, |a, b| Ordering::Equal);
}