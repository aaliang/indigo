use events::VectorStream;
use std::fmt::Debug;
use std::collections::LinkedList;

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
            if view.len() <= self.window_size {
                let init = Self::sub_sequences(&view[..], &self.max_to_get);
            } else {
                let mut windows = view.windows(self.window_size);
                match windows.next() {
                    Some(first_frame) => {
                        let init = Self::sub_sequences(first_frame, &self.max_to_get);
                        println!("{:?}", init);
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

    pub fn sub_sequences<A>(window: &[A], max_to_get: &usize) -> LinkedList<Vec<Vec<A>>> where A: Copy + Debug {
        //let mut window_state_list: Vec<Vec<Vec<A>>> = Vec::with_capacity(window.len());
        let mut window_state_list: LinkedList<Vec<Vec<A>>> = LinkedList::new();
        for element in window.iter().rev() {
            let mut new_entry = {
                window_state_list.iter()
                    .flat_map(|x| x.to_owned())
                    .map(|mut window_state_list| {
                        if &window_state_list.len() < max_to_get {
                            window_state_list.insert(0, *element);
                        }
                        window_state_list
                    })
                    .collect::<Vec<Vec<A>>>()
            };
            let singleton_el = vec![*element];
            new_entry.push(singleton_el);
            window_state_list.push_back(new_entry);
        }
        window_state_list
    }
}

pub struct GroupPipeline<'a> {
    stream: VectorStream<'a>,
    window_size: usize,
    max_to_get: usize
}

