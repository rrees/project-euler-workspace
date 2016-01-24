
fn main() {
	println!("Hello world");

	let mut result: u64 = 0;

	for i in (100..999) {
		for j in (100..999) {
			let product = i * j;

			if palindromic_number(product) && product > result {
				result = product;
			}
		}
	}

	println!("Largest palindromic product: {product}", product=result);
}

fn palindromic_number(i: u64) -> bool {
	let s = i.to_string();
	let reverse = s.chars().rev().collect::<String>();
	return s == reverse;
}
