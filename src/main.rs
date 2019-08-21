mod character;
mod dice;

fn main() {
	for i in 1..10 {
		let result = dice::roll(i, dice::RollType::Standard);
		println!("{:?}", result);
	}
}
