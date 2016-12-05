use object_mapper::MapBecome;

use events::name_hint::NamedIndexView;

impl EventHistory {
    fn new(line: &str) -> EventHistory {
        EventHistory {
            events: line.split(",").map(|name| name.into()).collect()
        }
    }
}

#[derive(Debug)]
struct EventHistory {
    events: Vec<String>
}

impl <'a> MapBecome<EventHistory> for EventSim<'a> {
    fn calc(&self, line: &str) -> EventHistory {
        EventHistory::new(line)
    }
}

pub struct EventSim <'a> {
    pub hint: NamedIndexView<'a>
}