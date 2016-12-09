#![feature(test)]

pub mod object_mapper;
pub mod events;
pub mod collector;
pub mod view;

extern crate time;
extern crate getopts;
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use events::groups::GroupPipeline;
    
    #[bench]
    fn bench_sub_sequence(b: &mut Bencher) { 
        
        let vec: Vec<u32> = (0..5).collect();
        println!("hello"); 
        b.iter(||GroupPipeline::sub_sequences(&vec, &3));
    }
}
