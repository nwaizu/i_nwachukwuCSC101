fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// depriciation
	let a = p * (1 - (r / 100.0)) * n;
    println!("Amount is {}", a);
}