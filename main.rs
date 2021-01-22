#[macro_use]
extern crate ferris_print;
extern crate ferris_says;
use rand::Rng;

 fn main() {
    ferrisprint!("Hello fellow Rustaceans!");
    let mut rng = rand::thread_rng();
		let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
		#[macro_use]
    ferrisprint!("Random u8: {}", n1);
		#[macro_use]
    ferrisprint!("Random u16: {}", n2);
  }
