use rand::Rng;
extern crate ferris_says;
use ferris_says::say;
use std::io::{ stdout, BufWriter };

	fn main() {
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
   

   
	  let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    
}

