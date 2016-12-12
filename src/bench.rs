extern crate indigo;

use indigo::events::groups::GroupPipeline;

extern crate time;

use std::fmt;
use std::fmt::Debug;
struct Stopwatch {
    split_time: Option<u64>,
    store_time: u64
}

impl Stopwatch {
    fn create_started() -> Stopwatch {
        Stopwatch {
            split_time: Some(time::precise_time_ns()),
            store_time: 0
        }
    }
    fn get_elapsed_time(&self) -> u64 {
        self.store_time + self.split_time.map_or(0, |time_ns| time::precise_time_ns() - time_ns)
    }
    fn reset(&mut self) {
        self.split_time = None;
        self.store_time = 0;
    }
    fn start(&mut self) {
        if self.split_time.is_none() {
          self.split_time = Some(time::precise_time_ns());
        }
    }
    fn pause(&mut self) {
        self.store_time += self.split_time.map_or(0, |time_ns| time::precise_time_ns() - time_ns);
        self.split_time = None;
    }
}

//fn trunc_spaces(n: u64, power: u32) -> f64 { n as f64 / (10f64).pow(power)}

/*fn time_format_for(n: u64) -> (f64, &'static str) {
    match n {
      n if n < 1_000 => (trunc_spaces(n, 0), "ns"),
      n if n < 1_000_000 => (trunc_spaces(n, 3), "us"),
      n if n < 1_000_000 => (trunc_spaces(n, 6), "ms"),
      n if n < 1_000_000 => (trunc_spaces(n, 9), "s"),
      n => (0, "ns")
    }

}*/


impl Debug for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elapsed_time = self.get_elapsed_time();
        let (display, unit) = match elapsed_time {
          e if e < 1_000 => (e, "ns"),
          e => (e, "ns") //brain is fried tonight. ill get real human readable things later
        };
        write!(f, "{}{}", self.get_elapsed_time(), unit)
    }
}



fn main() {
    bench_sub_sequence();
}

fn bench_sub_sequence() { 
    let vec: Vec<u32> = (0..20).collect();
    println!("hello"); 
    let returned = GroupPipeline::sub_sequences(&vec, &10);
}
