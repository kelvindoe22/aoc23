fn main() {
 	let whole_t = std::env::args().nth(1).expect("no arguments received");
	let mut lines:Vec<&str> = whole_t.split('\n').collect();
	let wr = |word: &str| {
		let mut string = String::new();
		for c in word.chars() {
			if c.is_digit(10) {
				string.push(c)
			}
		}
		return string;
	};
	let numbers = lines.into_iter().map(|x| wr(x)).collect::<Vec<_>>();
	let mut total = 0;
	for n in numbers.iter() {
		let mut string = String::new();
		string.push(n.chars().next().expect("Empty Text").clone());
		string.push(n.chars().last().expect("Empty Text").clone());
		total += string.parse::<usize>().expect("Parse Error");
	}
	println!("{}", total)

}

