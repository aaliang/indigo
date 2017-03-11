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
    chunk_size: u32
}

use std::fs::File;

struct FileIterator<I> {
    inner: Box<Iterator<Item=I>>,
    options: SortOptions,
    num: u32
}

impl <I> FileIterator<I> {
    fn new(inner: Box<Iterator<Item=I>>, options: SortOptions) -> FileIterator<I> {
        FileIterator {
            inner: inner,
            options: options,
            num: 0
        }
    }
}

// iterating over open files seems dangerous maybe? if the iterator is unspooled into a collection
// this could get messy.
impl <I> Iterator for FileIterator<I> where I: Serializable {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        use std::io::Write;

        let next = self.inner.next();
        match next {
            None => None,
            Some(value) => {
                let path = format!("{}/{}", self.options.directory, self.num);
                let mut fd = File::create(path).unwrap(); // TODO: error handling
                fd.write_all(value.serialize()).unwrap();

                for _ in 1..self.options.chunk_size {
                    match self.inner.next() {
                        None => break,
                        Some(val) => {
                            fd.write_all(val.serialize()).unwrap();
                        }
                    }
                }

                self.num += 1;
                Some(fd)
            }
        }
    }
}

pub struct SortMe;

impl SortMe {
    fn sort<I, W>(to_sort: I, options: SortOptions, sort_fn: Box<Fn(&W, &W) -> Ordering>) where I: Iterator<Item=W>, W: Serializable {
        //TODO
    }
}