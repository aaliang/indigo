#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;

pub trait Serializable {
    fn serialize(&self) -> &[u8];
    fn deserialize(raw: &[u8]) -> Self;
}

pub trait EventLike {
    fn get_event_ame(&self) -> &str;
    fn get_unique_id(&self) -> &str;
    fn get_time(&self) -> &u64;
}

pub struct Event {
    timestamp: u64,
    event_name: String,
    unique_id: String
}


pub struct SortOptions {
    directory: String,
    chunk_size: usize
}

use std::fs::File;

struct FileIterator<I> {
    inner: Box<Iterator<Item=I>>,
    options: SortOptions,
    num: usize,
    sort_by: Box<FnMut(&I, &I) -> Ordering>
}

impl <I> FileIterator<I> {
    fn new<F>(inner: Box<Iterator<Item=I>>, options: SortOptions, sort_fn: F) -> FileIterator<I> where F: FnMut(&I, &I) -> Ordering + 'static {
        FileIterator {
            inner: inner,
            options: options,
            num: 0,
            sort_by: Box::new(sort_fn)
        }
    }
}

// iterating over open files seems dangerous maybe? if the iterator is unspooled into a collection
// this could get messy.
impl <I> Iterator for FileIterator<I> where I: Serializable {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        use std::io::Write;

        let mut chunk = self.inner
            .by_ref()
            .take(self.options.chunk_size)
            .collect::<Vec<_>>();

        if chunk.is_empty() {
            None
        } else {

            let path = format!("{}/{}", self.options.directory, self.num);
            let mut fd = File::create(path).unwrap(); // TODO: error handling
            chunk.sort_by(|a, b| (self.sort_by)(a, b));

            for value in chunk {
                fd.write_all(value.serialize()).unwrap()
            }

            self.num += 1;

            Some(fd)
        }
    }
}

pub struct SortMe;

impl SortMe {
    fn sort<I, F>(to_sort: Box<Iterator<Item=I>>, options: SortOptions, sort_fn: F)
        where I: Serializable, F: FnMut(&I, &I) -> Ordering + 'static {
        use std::io::{BufRead, BufReader};
        use std::io::Lines;

        let files = FileIterator::new(to_sort, options, sort_fn);

        let my_vec = files.map(|file| {
            let reader = BufReader::new(file);
            let f = reader.lines().map(|line| I::deserialize(line.unwrap().as_bytes()));
            f
        }).collect::<Vec<_>>();

        // this is sort of a bad idea
//        let vec: Vec<File> = files.collect();
    }
}

