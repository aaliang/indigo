pub mod groups;
pub mod name_hint;

use events::name_hint::NamedIndexView;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines, Read, Error};

impl <'a> VectorStream<'a> {

    pub fn from<'b> (hint: &'a str, group_path: &'b str) -> VectorStream<'a> {
        let hint_view = NamedIndexView::new(hint);
        let fd = File::open(group_path).unwrap();
        let file = BufReader::new(fd);

        VectorStream {
            reader: file,
            i_view: hint_view.unwrap()
        }
    }

    pub fn try_get(&mut self) -> Result<Vec<u32>, Error> {
        let mut into = [0; 4];
        try!(self.reader.read_exact(&mut into));
        let group_size = as_u32_be(&into);
        let mut returned_vector = Vec::with_capacity(group_size as usize);
        for _ in 1..group_size {
            try!(self.reader.read_exact(&mut into));
            returned_vector.push(as_u32_be(&into));
        }
        Ok(returned_vector)
    }

    pub fn get_next(&mut self) -> Option<Vec<u32>> {
        //self.try_get().ok()
        match self.try_get() {
            Err(e) => {
                println!("{:?}", e);
                None
            },
            Ok(d) => Some(d)
        }
    }
}

pub struct VectorStream<'a> {
    reader: BufReader<File>,
    i_view: NamedIndexView<'a>
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) |
    ((array[1] as u32) << 16) |
    ((array[2] as u32) <<  8) |
    ((array[3] as u32) <<  0)
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) |
    ((array[1] as u32) <<  8) |
    ((array[2] as u32) << 16) |
    ((array[3] as u32) << 24)
}

impl <'a> Iterator for VectorStream<'a> {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Vec<u32>> {
        self.get_next()
    }
}
