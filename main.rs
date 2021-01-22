#![allow(unused)]
#[macro_use]
extern crate ferris_print;
use rand::Rng;
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("1").spawn().unwrap();
    let _result = child.wait().unwrap();
    ferrisprint!("Hello fellow Rustaceans!");
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    let mut child = Command::new("sleep").arg("3").spawn().unwrap();
    let _result = child.wait().unwrap();
  #[macro_use]
    ferrisprint!("Random u8: {}", n1);
    let mut child = Command::new("sleep").arg("3").spawn().unwrap();
    let _result = child.wait().unwrap();
  #[macro_use]
    ferrisprint!("Random u16: {}", n2);
}
