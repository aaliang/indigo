use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error, Lines};

/// while this seemed like a promising abstraction, this is considered unused in favor of
/// VectorStream in stream.rs
/// perhaps this will be useful again -- Dec 05, 2016

pub trait MapBecome<A> {
    fn becomes(&self, s: &str) -> A;
}

pub struct ObjectMapper<A> {
    lines: Lines<BufReader<File>>,
    mapper: Box<MapBecome<A>>
}

impl <A> ObjectMapper<A> {
    pub fn new<'a, M> (path: &'a str, mapper: M) -> Result<ObjectMapper<A>, Error> where M: 'static + MapBecome<A> {
        let fd = try!(File::open(path));
        let file = BufReader::new(fd);
        Ok(ObjectMapper {
            lines: file.lines(),
            mapper: Box::new(mapper)
        })
    }
}

// support passing a closure too - doesn't quite work yet
impl <A, C> MapBecome<A> for Box<C> where C: Fn(&str) -> A {
    fn becomes(&self, s: &str) -> A {
        (self)(s)
    }
}

impl <A> Iterator for ObjectMapper<A> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        let line = self.lines.next();
        line.map(|l| self.mapper.becomes(&l.unwrap()))
    }
}