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
    sort_by: Box<Fn(&I, &I) -> Ordering>
}

impl <I> FileIterator<I> {
    fn new<F>(inner: Box<Iterator<Item=I>>, options: SortOptions, sort_fn: F) -> FileIterator<I> where F: Fn(&I, &I) -> Ordering + 'static {
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
            // writing should probably be done in a sink thread
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
    fn sort<I, F>(to_sort: Box<Iterator<Item=I>>, options: SortOptions, chunk_sort: F, merge_sort: F)
        where I: Serializable + Clone,
              F: Fn(&I, &I) -> Ordering + 'static {
        use std::io::{BufRead, BufReader};
        use std::io::Lines;

        let files = FileIterator::new(to_sort, options, chunk_sort);

        let mut my_vec = files.map(|file| {
            let p = BufReader::new(file)
                .lines()
                .map(|line| I::deserialize(line.unwrap().as_bytes()));
            IHead::new(p)
        }).collect::<Vec<_>>();

        let i_head_max = my_vec.iter_mut()
            .max_by(|a, b| {
                let ref a_head = a.head;
                let ref b_head = b.head;
                match (a_head, b_head) {
                    (&None, &None) => Ordering::Equal,
                    (&Some(_), &None) => Ordering::Greater,
                    (&None, &Some(_)) => Ordering::Less,
                    (&Some(ref _a), &Some(ref _b)) => (merge_sort)(_a, _b)
                }
            });

        match i_head_max {
            None => None,
            Some(thing) => {
                let emit_me = thing.advance();
                emit_me
            }
        };
    }
}

struct IHead<I> {
    iterator: Box<Iterator<Item=I>>,
    head: Option<I>
}


//TODO: need to use Cell or I needs to be Clone
impl <I> IHead<I> where I: Clone {
    fn new<A>(mut iterator: A) -> IHead<I> where A: Iterator<Item=I> + 'static {
        let head = iterator.next();

        IHead {
            iterator: Box::new(iterator),
            head: head
        }
    }
    fn advance(&mut self) -> Option<I> {
        let head = match self.head {
            Some(ref e) => Some(e.to_owned()),
            None => None
        };
        self.head = self.iterator.next();
        head
    }
}

