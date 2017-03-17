#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::io::{Write, Read, Error};

pub trait Deserialize: Sized {
    fn deserialize(reader: &mut Read) -> Result<Option<Self>, Error>;
}

pub trait Serialize {
    /// serializes self to the writer. resulting in the number of bytes written
    fn serialize(&self, writer: &mut Write) -> Result<usize, Error>;
}

use std::marker::PhantomData;

struct StreamDeserializer<I, R: Read + Sized> {
    phantom: PhantomData<I>,
    reader: R
}

impl <I, R: Read + Sized> StreamDeserializer<I, R> {
    pub fn new(r: R) -> StreamDeserializer<I, R> {
        StreamDeserializer {
            phantom: PhantomData,
            reader: r
        }
    }
}

impl <I, R: Read + Sized>Iterator for StreamDeserializer<I, R> where I: Deserialize {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let result = I::deserialize(&mut self.reader);
        match result {
            Err(_) => None,
            Ok(item) => item
        }
    }
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
use std::fmt::Debug;
// iterating over open files seems dangerous maybe? if the iterator is unspooled into a collection
// this could get messy.
impl <'a, I> Iterator for FileIterator<'a, I> where I: Serialize + Debug {
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
//                println!("writing to {} [{:?}]", self.num, value);
                value.serialize(&mut fd).unwrap();
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

impl <E> ExternalSort<E> where E: Clone + Serialize + Deserialize + Debug + 'static {
    pub fn new<F>(to_sort: Box<Iterator<Item=E>>, options: SortOptions, chunk_sort: F)
        -> ExternalSort<E>
        where F: Fn(&E, &E) -> Ordering + 'static {
        use std::io::{BufRead, BufReader};
        use std::io::Lines;

        let sorted_chunks = {
            let paths = FileIterator::new(to_sort, options, &chunk_sort);
            paths.map(|file| {
                let fd = BufReader::new(File::open(file).unwrap());
                let ds = StreamDeserializer::new(fd);
                IHead::new(ds)
            }).collect::<Vec<_>>()
        };

        ExternalSort {
            sorted_chunks: sorted_chunks,
            sort_fn: Box::new(chunk_sort)
        }
    }
}

impl <I> Iterator for ExternalSort<I> where I: Clone + Debug {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let ref sort_fn = self.sort_fn;
        let head_min = self.sorted_chunks.iter_mut()
            .min_by(|a, b| {
                let ref a_head = a.head;
                let ref b_head = b.head;
                match (a_head, b_head) {
                    (&None, &None) => Ordering::Equal,
                    (&Some(_), &None) => Ordering::Less,
                    (&None, &Some(_)) => Ordering::Greater,
                    (&Some(ref _a), &Some(ref _b)) => (sort_fn)(_a, _b)
                }
            });

        match head_min {
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

impl <I> IHead<I> where I: Clone + Debug {
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
    let boxed_seq: Box<Iterator<Item=u32>> = Box::new(1..1000);

    use std::fs;

    fs::create_dir_all("/tmp/r-sort/ns-1").unwrap();

    let options = SortOptions {
        directory: "/tmp/r-sort/ns-1".to_string(),
        chunk_size: 100
    };

    let es = ExternalSort::new(boxed_seq, options, |a, b| Ordering::Equal);

    for i in es {
        println!("{}", i);
    }
}

impl Deserialize for u32 {
    fn deserialize (reader: &mut Read) -> Result<Option<Self>, Error> {
        let mut buffer: [u8; 4] = [0; 4];
        reader.read_exact(&mut buffer).map(|_| {
            let mut num: u32 = 0;
            num |= buffer[3] as u32;
            num |= (buffer[2] as u32) << 8;
            num |= (buffer[1] as u32) << 16;
            num |= (buffer[0] as u32) << 24;
            Some(num)
        })
    }
}

impl Serialize for u32 {
    fn serialize (&self, writer: &mut Write) -> Result<usize, Error> {
        let b1 = ((self >> 24u32) & 0xffu32) as u8;
        let b2 = ((self >> 16u32) & 0xffu32) as u8;
        let b3 = ((self >> 8u32) & 0xffu32) as u8;
        let b4 = (self & 0xffu32) as u8;
        writer.write_all(&[b1, b2, b3, b4]).map(|_| 4)
    }
}