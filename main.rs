#[macro_use]
extern crate ferris_print;
extern crate ferris_says;
use ferris_says::say;
use std::io::{ stdout, BufWriter };
use rand::Rng;

 fn main() {
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
		let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
   	let mut rng = rand::thread_rng();
		let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
		#[macro_use]
    ferrisprint!("Random u8: {}", n1);
		#[macro_use]
    ferrisprint!("Random u16: {}", n2);
  }