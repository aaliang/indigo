#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;

pub trait Serializable {
    //TODO: could probably avoid an allocation altogether by passing a Write impl
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(raw: &[u8]) -> Self;
}

pub trait EventLike {
    fn get_event_name(&self) -> &str;
    fn get_unique_id(&self) -> &str;
    fn get_time(&self) -> &u64;
}

pub struct Event {
    timestamp: u64,
    event_name: String,
    unique_id: String
}

pub struct SortOptions {
    pub directory: String,
    pub chunk_size: usize
}

use std::fs::File;

struct FileIterator<'a, I: 'a> {
    inner: Box<Iterator<Item=I>>,
    options: SortOptions,
    num: usize,
    sort_by: &'a Fn(&I, &I) -> Ordering
}

impl <'a, I> FileIterator<'a, I> {
    fn new<F>(inner: Box<Iterator<Item=I>>, options: SortOptions, sort_fn: &'a F) -> FileIterator<'a, I>
        where F: Fn(&I, &I) -> Ordering + 'a {
        FileIterator {
            inner: inner,
            options: options,
            num: 0,
            sort_by: sort_fn
        }
    }
}


use std::path::{Path, PathBuf};
// iterating over open files seems dangerous maybe? if the iterator is unspooled into a collection
// this could get messy.
impl <'a, I> Iterator for FileIterator<'a, I> where I: Serializable {
    type Item = PathBuf; // path to file. it could just be string, but let's wrap it to make it more semantic
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
            let mut fd = File::create(&path).unwrap(); // TODO: error handling
            chunk.sort_by(|a, b| (self.sort_by)(a, b));
            // writing should probably be done in a sink thread
            for value in chunk {
                fd.write_all(&value.serialize()).unwrap();
                fd.write_all(b"\n").unwrap();
            }
            self.num += 1;
            Some(PathBuf::from(path))
        }
    }
}

pub struct ExternalSort<I: Clone> {
    sorted_chunks: Vec<IHead<I>>,
    sort_fn: Box<Fn(&I, &I) -> Ordering>
}

impl <I> ExternalSort<I> where I: Clone + Serializable {
    pub fn new<F>(to_sort: Box<Iterator<Item=I>>, options: SortOptions, chunk_sort: F)
        -> ExternalSort<I>
        where F: Fn(&I, &I) -> Ordering + 'static {
        use std::io::{BufRead, BufReader};
        use std::io::Lines;

        let sorted_chunks = {
            let paths = FileIterator::new(to_sort, options, &chunk_sort);
            paths.map(|file| {
                let p = BufReader::new(File::open(file).unwrap())
                    .split('\n' as u8) // this is dangerous as fuck. and incorrect. splitting is not the way to go
                    .map(|line| {
                        let l = line.unwrap();
                        I::deserialize(&l)
                    });
                IHead::new(p)
            }).collect::<Vec<_>>()
        };

        ExternalSort {
            sorted_chunks: sorted_chunks,
            sort_fn: Box::new(chunk_sort)
        }
    }
}

impl <I> Iterator for ExternalSort<I> where I: Clone {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let ref sort_fn = self.sort_fn;
        let head_max = self.sorted_chunks.iter_mut()
            .max_by(|a, b| {
                let ref a_head = a.head;
                let ref b_head = b.head;
                match (a_head, b_head) {
                    (&None, &None) => Ordering::Equal,
                    (&Some(_), &None) => Ordering::Greater,
                    (&None, &Some(_)) => Ordering::Less,
                    (&Some(ref _a), &Some(ref _b)) => (sort_fn)(_a, _b)
                }
            });

        match head_max {
            None => None,
            Some(thing) => {
                let emit_me = thing.advance();
                emit_me
            }
        }
    }
}

struct IHead<I> {
    iterator: Box<Iterator<Item=I>>,
    head: Option<I>
}

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

#[test]
fn test() {

    impl Serializable for u32 {
        fn serialize(&self) -> Vec<u8> {
            let b1 = ((self >> 24u32) & 0xffu32) as u8;
            let b2 = ((self >> 16u32) & 0xffu32) as u8;
            let b3 = ((self >> 8u32) & 0xffu32) as u8;
            let b4 = (self & 0xffu32) as u8;
            vec![b1, b2, b3, b4]
        }

        fn deserialize(raw: &[u8]) -> Self {
            unimplemented!()
        }
    }


    let boxed_seq: Box<Iterator<Item=u32>> = Box::new(1..1000);

    use std::fs;

    fs::create_dir_all("/tmp/r-sort/ns-1").unwrap();

    let options = SortOptions {
        directory: "/tmp/r-sort/ns-1".to_string(),
        chunk_size: 100
    };

    ExternalSort::new(boxed_seq, options, |a, b| Ordering::Equal);
}

// this is temporary shitcode to get around reconciling splitting on the bufreader
impl Serializable for u32 {
    fn serialize(&self) -> Vec<u8> {
        let s = self.to_string();
        s.as_bytes().to_owned()
    }

    fn deserialize(raw: &[u8]) -> Self {
        let as_string = String::from_utf8_lossy(raw);
        as_string.parse::<u32>().unwrap()
    }
}

//impl Serializable for u32 {
//    fn serialize(&self) -> Vec<u8> {
//        let b1 = ((self >> 24u32) & 0xffu32) as u8;
//        let b2 = ((self >> 16u32) & 0xffu32) as u8;
//        let b3 = ((self >> 8u32) & 0xffu32) as u8;
//        let b4 = (self & 0xffu32) as u8;
//        vec![b1, b2, b3, b4]
//    }
//
//    fn deserialize(raw: &[u8]) -> Self {
//        let mut num: u32 = 0;
//        num += raw[3] as u32;
//        num += (raw[2] as u32) << 4;
//        num += (raw[1] as u32) << 8;
//        num += (raw[0] as u32) << 12;
//        num
//    }
//}
