use rand::Rng;
 

fn main(){
	loop {
	let mut rng = rand::thread_rng();
	let r1: u8 = rng.gen();
	println! ("num {}", r1);
}
}