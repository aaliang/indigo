use events::name_hint::NamedIndexView;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

impl <'a> VectorStream<'a> {

    pub fn from<'b> (hint: &'a str, group_path: &'b str) -> VectorStream<'a> {
        let hint_view = NamedIndexView::new(hint);
        let fd = File::open(group_path).unwrap();
        let file = BufReader::new(fd);

        VectorStream {
            lines: file.lines(),
            i_view: hint_view.unwrap()
        }
    }
}

pub struct VectorStream<'a> {
    lines: Lines<BufReader<File>>,
    i_view: NamedIndexView<'a>
}

impl <'a> Iterator for VectorStream<'a> {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let line_opt = self.lines.next();
        line_opt.map(|line| {
            line.unwrap()
                .split(",")
                .map(|name| self.i_view.translate(name).unwrap().to_owned())
                .collect()
        })
    }
}