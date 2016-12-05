pub trait Sliding<A> {
    fn slide(&mut self) -> Option<&[A]>;
}

impl <A> Slider<A> {
    pub fn new(basis: Vec<A>, window_size: usize) -> Slider<A> {
        Slider {
            basis: basis,
            head_idx: 0,
            last_idx: window_size
        }
    }
}
pub struct Slider<A> {
    basis: Vec<A>,
    head_idx: usize,
    last_idx: usize
}

impl <A> Sliding<A> for Slider<A> {
    fn slide(&mut self) -> Option<&[A]> {
        if self.head_idx < self.basis.len() {
            let view = if self.last_idx <= self.basis.len() {
                &self.basis[self.head_idx..self.last_idx]
            } else {
                &self.basis[self.head_idx..self.basis.len()]
            };

            self.head_idx += 1;
            self.last_idx += 1;

            Some(view)
        } else {
            None
        }

    }
}