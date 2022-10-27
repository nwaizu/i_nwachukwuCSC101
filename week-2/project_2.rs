fn main() {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;

	// sum
	let s = toshiba + mac + hp + dell + acer;
	println!("Sum is {}", s);
	let a = s / 5.0;
	println!("Average is {}", a);
}