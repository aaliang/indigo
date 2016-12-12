pub trait Sliding<A> {
    fn slide(&mut self) -> Option<&[A]>;
}

macro_rules! slide_view {
    ($this:expr) => ({
            let view = if $this.last_idx <= $this.basis.len() {
            &$this.basis[$this.head_idx..$this.last_idx]
        } else {
            &$this.basis[$this.head_idx..$this.basis.len()]
        };
        view
    });
}

//trait ListLike<A> {
//    fn head(&self) -> Option<&A>;
//    fn tail(&self) -> &[A];
//}
//
//impl ListLike<A> for &[A] {
//    fn head(&self) -> {
//        self.first()
//    }
//
//}

impl <A> VecView<A> {

    pub fn new(basis: Vec<A>, window_size: usize) -> VecView<A> {
        VecView {
            basis: basis,
            head_idx: 0,
            last_idx: window_size
        }
    }

    pub fn view(&self) -> &[A] {
        slide_view!(&self)
    }
}

pub struct VecView<A> {
    basis: Vec<A>,
    head_idx: usize,
    last_idx: usize
}

impl <A> Sliding<A> for VecView<A> {

    fn slide(&mut self) -> Option<&[A]> {
        if self.head_idx < self.basis.len() {
            let view = slide_view!(self);
            self.head_idx += 1;
            self.last_idx += 1;
            Some(view)
        } else {
            None
        }
    }
}