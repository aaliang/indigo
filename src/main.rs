extern crate indigo;

use indigo::events::stream::VectorStream;
use indigo::events::name_hint::NameHint;
use indigo::events::history::History;

fn main() {

    let name_path = "/home/andy/dev/indigo/d60bcc93-f0b9-11e2-b49c-002590d151de.2016-11-27T00:00:00.000Z,2016-11-28T00:00:00.000Z.eventNames";

    let hint: String = NameHint::ld_string(name_path).unwrap();

    let stream = VectorStream::from(
        &hint,
        "/home/andy/dev/indigo/d60bcc93-f0b9-11e2-b49c-002590d151de.2016-11-27T00:00:00.000Z,2016-11-28T00:00:00.000Z.groups");

    let hist = History::new(stream, 10, 5);
    hist.mine();
}
