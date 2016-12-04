use object_mapper::MapBecome;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use collector::NamedIndex;

pub struct EventSim {}

impl MapBecome<EventHistory> for EventSim {
    fn calc(&self, line: &str) -> EventHistory {
        EventHistory {
            events: line.split(",").map(|name| name.into()).collect()
        }
    }
}

#[derive(Debug)]
struct EventHistory {
    events: Vec<String>
}

fn from_hint_file(path: &str) -> Result<HashSet<String>, Error> {
    let fd = try!(File::open(path));
    let mut file = BufReader::new(fd);
    let mut str = String::new();
    let _ = file.read_to_string(&mut str);

    Ok(str.split(",").map(|name| name.into()).collect())
}

pub struct NameHint {
    index: HashMap<String, usize>,
    reverse_index: Vec<String>
}

impl NameHint {

    pub fn from(path: &str) -> Result<NameHint, Error> {
        let fd = try!(File::open(path));
        let mut file = BufReader::new(fd);
        let mut str = String::new();
        let _ = file.read_to_string(&mut str);

        let index = NamedIndex::to_index(str.split(","));
        let reverse_index = NamedIndex::to_vec(&index);

        println!("{:?}", reverse_index);
        println!("{:?}", index);

        Ok(NameHint {
            index: index,
            reverse_index: reverse_index
        })
    }
}