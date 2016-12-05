use std::collections::HashMap;

impl NamedIndex {

    pub fn to_vec (hash_map: &HashMap<String, usize>) -> Vec<String> {
        let len = hash_map.len();
        let mut vector = Vec::with_capacity(len);
        unsafe { vector.set_len(len) };
        for (h, idx) in hash_map {
            vector[*idx] = h.to_owned();
        }
        vector
    }

    pub fn to_index <'a, I>(iter: I) -> HashMap<String, usize> where I: Iterator<Item=&'a str> + Sized {
        let mut index = HashMap::new();
        for tok in iter {
            if ! index.contains_key(tok) {
                let len = index.len();
                index.insert(tok.into(), len);
            }
        }
        index
    }
}

pub struct NamedIndex;
//
//impl<'a> NamedIndexViewB<'a> {
//
//    pub fn new(view: String) -> (Result<NamedIndexViewB<'a>, &'static str> {
//        let lookup_index: Vec<&str> = (&view).split(",").collect();
//
//        let name_index: HashMap<&str, usize> = lookup_index.iter()
//            .enumerate()
//            .map(|(a, b)| (*b, a)).collect();
//
//        if name_index.len() != lookup_index.len() {
//            Err("view must be a csv of uniques")
//        } else {
//            Ok(NamedIndexViewB {
//                view: view,
//                comp_index: lookup_index,
//                name_index: name_index
//            })
//        }
//    }
//
//    pub fn translate(&self, name: &str) -> Option<&usize>{
//        self.name_index.get(name)
//    }
//
//    pub fn name_for(&self, index: usize) -> Option<&str>{
//        if index < self.comp_index.len() {
//            Some(self.comp_index[index])
//        } else {
//            None
//        }
//    }
//}
//
//pub struct NamedIndexViewB<'a> {
//    view: String,
//    comp_index: Vec<&'a str>,
//    name_index: HashMap<&'a str, usize>
//}