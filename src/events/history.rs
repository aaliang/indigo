use object_mapper::MapBecome;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};

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

pub struct NameHint;

impl NameHint {
    pub fn fromHintFile(path: &str) -> Result<HashSet<String>, Error> {
        let fd = try!(File::open(path));
        let mut file = BufReader::new(fd);
        let mut str = String::new();
        let l = file.read_to_string(&mut str);

        Ok(str.split(",").map(|name| name.into()).collect())
    }
}