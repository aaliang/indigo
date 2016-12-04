extern crate indigo;

use indigo::object_mapper::{ObjectMapper};
use indigo::events::history::{EventSim, NameHint};

fn main() {
    let fr = ObjectMapper::new(
        "/tmp/d60bcc93-f0b9-11e2-b49c-002590d151de.2016-11-27T00:00:00.000Z,2016-11-28T00:00:00.000Z.groups",
        EventSim {});
    let ns = NameHint::fromHintFile("/tmp/d60bcc93-f0b9-11e2-b49c-002590d151de.2016-11-27T00:00:00.000Z,2016-11-28T00:00:00.000Z.eventNames");

    println!("{:?}", ns);


    match fr {
        Ok(lr) => {
            for n in lr {
//                println!("{:?}", n);
            }
        },
        Err(e) => println!("no, {:?}", e)
    }
}
