
fn main() {
	println!("Hello world");
	println!("Sum: {sum}", sum=problem2(1, 2, 0));
}

fn problem2(a: u64, b: u64, sum: u64) -> u64 {
	if b > 4000000 {
		return sum;
	}

	if b % 2 == 0 {
		return problem2(b, a + b, sum + b)
	} else {
		return problem2(b, a + b, sum)
	}
}
