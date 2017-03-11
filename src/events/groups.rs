use events::VectorStream;
use std::fmt::Debug;
use std::collections::LinkedList;

pub struct GroupPipeline<'a> {
    stream: VectorStream<'a>,
    window_size: usize,
    max_to_get: usize
}

impl <'a> GroupPipeline <'a> {

    pub fn new(stream: VectorStream<'a>, window_size: usize, max_positions: usize) -> GroupPipeline<'a> {
        GroupPipeline {
            stream: stream,
            window_size: window_size,
            max_to_get: max_positions
        }
    }

    fn produce<W, A: Copy + 'a>(iter: W) -> () where W: Iterator<Item=&'a mut Vec<Vec<A>>> {
        for outer_vec in iter {
            for inner_vec in outer_vec.iter_mut() {
                let _ = inner_vec.clone();
            }
        }
    }

    pub fn mine (self) {
        for view in self.stream {
            if view.len() <= self.window_size {
                let _ = Self::sub_sequences(&view[..], &self.max_to_get);
            } else {
                let initial_window = &view[..self.window_size];
                let mut initial_seqs = Self::sub_sequences(initial_window, &self.max_to_get);
                let rest_elements = view.iter().skip(10);


                for element in rest_elements {
                    //println!("{:?}", element);
                    let _ = Self::produce(initial_seqs.iter_mut());
                    let _ = initial_seqs.push_back(vec![vec![*element]]);
                    let _ = initial_seqs.pop_front();
                }
            }
        }
        println!("we done here");
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
