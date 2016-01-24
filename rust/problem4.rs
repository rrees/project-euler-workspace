
fn main() {
	println!("Hello world");
	println!("Largest palindromic product: {product}", product=problem4(999, 999, 0));
}

fn palindromic_number(i: u64) -> bool {
	let s = i.to_string();
	let reverse = s.chars().rev().collect::<String>();
	return s == reverse;
}

fn problem4(a: u64, b: u64, max_product: u64) -> u64 {
	if a < 100 || b < 100 {
		println!("Termination!");
		return max_product;
	}

	let product = a * b;
	println!("{a} {b} {p} {m} {pal}", a=a, b=b, p=product, m=max_product, pal=palindromic_number(product));

	if palindromic_number(product) && product > max_product {
		return problem4(a-1, b-1, product);
	} else {
		return problem4(a-1, b-1, max_product);
	}
}
