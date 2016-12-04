extern crate indigo;

use indigo::file_reader::{ObjectMapper, MapTrait};

struct EventSim;

impl MapTrait<String> for EventSim {
    fn calc(&self, line: &str) -> String {
        line.to_string()
    }
}

fn main() {
    let fr = ObjectMapper::new(
        "/tmp/something",
        EventSim {});


    match fr {
        Ok(mut lr) => {
            for n in lr {
                println!("{:?}", n);
            }
        },
        Err(e) => println!("no, {:?}", e)
    }
}
