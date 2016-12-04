use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error, Lines};

pub trait MapBecome<A> {
    fn calc(&self, s: &str) -> A;
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
    fn calc(&self, s: &str) -> A {
        (self)(s)
    }
}

impl <A> Iterator for ObjectMapper<A> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        let line = self.lines.next();
        line.map(|l| self.mapper.calc(&l.unwrap()))
    }
}