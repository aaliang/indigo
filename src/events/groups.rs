use object_mapper::MapBecome;
use events::VectorStream;

use view::VecView;
use std::fmt::Debug;

use events::name_hint::NamedIndexView;

impl <'a> GroupPipeline <'a> {

    pub fn new(stream: VectorStream<'a>, window_size: usize, max_positions: usize) -> GroupPipeline<'a> {
        GroupPipeline {
            stream: stream,
            window_size: window_size,
            max_to_get: max_positions
        }
    }

    pub fn mine (self) {
        for view in self.stream.take(10) {
            println!("{:?}", view);
            if view.len() <= self.window_size {
                println!("in here");
                let init = Self::sub_sequences(&view[..], &self.max_to_get);
            } else {
                let mut windows = view.windows(self.window_size);
                match windows.next() {
                    Some(first_frame) => {
                        let init = Self::sub_sequences(first_frame, &self.max_to_get);
                        for window in windows {
                            //                println!("{:?}", window);
                            //                Self::sub_sequences(window, &self.max_to_get);
                        }
                    },
                    None => println!("nothing here")
                };
            }
        }
    }

    pub fn sub_sequences<A>(window: &[A], max_to_get: &usize) -> Vec<Vec<Vec<A>>> where A: Copy + Debug {
        let mut vector: Vec<Vec<Vec<A>>> = Vec::with_capacity(window.len());

        //unsafe {vector.set_len(window.len())};

        for (vec_len, element) in window.iter().enumerate().rev() {
            let mut v:Vec<Vec<A>> = Vec::new();
            let mut new_entry = {
                vector.iter()
                    .flat_map(|x| x.to_owned())
                    .map(|mut vector| {
                        if &vector.len() < max_to_get {
                            vector.insert(0, *element);
                        }
                        vector
                    })
                    .collect::<Vec<Vec<A>>>()
            };
            let singleton_el = vec![*element];
            new_entry.push(vec![*element]);
            vector.push(new_entry);
        }
        vector
    }
}

pub struct GroupPipeline<'a> {
    stream: VectorStream<'a>,
    window_size: usize,
    max_to_get: usize
}

