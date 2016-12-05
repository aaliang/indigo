use std::fs::File;
use std::io::{Error, Read};
use std::collections::HashMap;

/// NamedIndexes are type identifiers and translators
impl<'a> NamedIndexView<'a> {

    /// view is a slice into a csv string (comma delimited). each entry in the csv is a type name
    /// encountered names are replaced with usizes.
    pub fn new(view: &'a str) -> Result<NamedIndexView<'a>, &str> {
        let lookup_index: Vec<&str> = view.split(",").collect();

        let name_index: HashMap<&str, usize> = lookup_index.iter()
            .enumerate()
            .map(|(a, b)| (*b, a)).collect();

        if name_index.len() != lookup_index.len() {
            Err("view must be a csv of uniques")
        } else {
            Ok(NamedIndexView {
                comp_index: lookup_index,
                name_index: name_index
            })
        }
    }

    /// Translates a name to the usize id if possible
    pub fn translate(&self, name: &str) -> Option<&usize>{
        self.name_index.get(name)
    }

    /// Gets the name of the id if possible
    pub fn name_for(&self, index: usize) -> Option<&str>{
        if index < self.comp_index.len() {
            Some(self.comp_index[index])
        } else {
            None
        }
    }
}

pub struct NamedIndexView<'a> {
    comp_index: Vec<&'a str>, // id -> name
    name_index: HashMap<&'a str, usize> // name -> id
}

impl NameHint {

    /// Loads the value of a string on file onto the heap
    pub fn ld_string(path: &str) -> Result<String, Error> {

        let mut fd = try!(File::open(path));

        let mut string = String::new();
        let _ = fd.read_to_string(&mut string);

        Ok(string)
    }

    /// Turns a raw value into a view
    pub fn hint_view<'a> (val: &'a str) -> NamedIndexView<'a> {
        NamedIndexView::new(val).unwrap()
    }

}

pub struct NameHint {}