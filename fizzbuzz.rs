fn main() {
	for num in range(1, 100) {
		println(
			// let's do that via match

			// _ is a the wildcard pa
			match num {
				_ if num % 15 == 0 => ~ "FizzBuzz",
				_ if num % 3 == 0 => ~ "Fizz",
				_ if num % 5 == 0 => ~ "Buzz",
				_ => num.to_str()
			}
		);
	}
}