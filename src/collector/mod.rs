use std::collections::HashMap;

pub struct NamedIndex;

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

    pub fn to_index <'a, I> (iter: I) -> HashMap<String, usize>
        where I: Iterator<Item=&'a str> + Sized {

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