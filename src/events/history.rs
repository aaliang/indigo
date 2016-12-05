use object_mapper::MapBecome;
use events::stream::VectorStream;
use sliding::{Slider, Sliding};

use events::name_hint::NamedIndexView;

impl <'a> History<'a> {

    pub fn new(stream: VectorStream<'a>, window_size: usize, max_positions: usize) -> History<'a> {
        History{
            stream: stream
        }
    }

    pub fn mine (self) {
        for i in self.stream {
            let something = Self::history(i);
//            println!();
        }
    }

    fn history(history: Vec<usize>) {
        let tot_len = history.len();
        let window_size = 8;
        let mut slider = Slider::new(history, window_size);

        loop {
            match slider.slide() {
                None => break,
                Some(s) => println!("{:?}", s)
            };
        }
    }
}

//#[derive(Debug)]
pub struct History<'a> {
    stream: VectorStream<'a>
}
