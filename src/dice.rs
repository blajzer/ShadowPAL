extern crate rand;
use rand::distributions::{Distribution, Uniform};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Glitch {
	None,
	Glitch,
	CriticalGlitch
}

#[derive(Debug)]
pub struct RollResult {
	dice: Vec<u8>,
	reroll_dice: Vec<u8>,
	hits: usize,
	glitch: Glitch,
	roll_type: RollType
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RollType {
	Standard,
	ReRollSixes,
	ReRollMisses,
	GlitchOnOneOrTwo,
}

pub fn roll(dice_count: usize, roll_type: RollType) -> RollResult {
	let mut rng = rand::thread_rng();
	let die = Uniform::new_inclusive(1, 6);

	let mut dice = Vec::new();
	let mut reroll_dice = Vec::new();
	let mut glitch_count = 0;
	let mut hits = 0;

	// Standard roll
	for _ in 0..dice_count {
		let roll = die.sample(&mut rng);
		dice.push(roll);

		if roll == 5 || roll == 6 {
			hits += 1;
		} else if roll == 1 || (roll == 2 && roll_type == RollType::GlitchOnOneOrTwo) {
			glitch_count += 1;
		}
	}

	// "Push the Limit"
	// Re-roll all 6s, continuously (often called "exploding dice rules")
	if roll_type == RollType::ReRollSixes {
		for &original in dice.iter() {
			let mut roll = original;
			while roll == 6 {
				roll = die.sample(&mut rng);
				reroll_dice.push(roll);

				if roll == 5 || roll == 6 {
					hits += 1;
				} else if roll == 1 {
					glitch_count += 1;
				}
			}
		}
	}

	// Evaluate glitch status
	let glitch = if glitch_count > (dice_count + reroll_dice.len()) / 2 {
		if hits == 0 { Glitch::CriticalGlitch } else { Glitch::Glitch }
	} else {
		Glitch::None
	};

	// "Second Chance"
	// Re-roll misses, but can't negate glitch/critical glitches
	if roll_type == RollType::ReRollMisses {
		for _ in 0..(dice_count - hits) {
			let roll = die.sample(&mut rng);
			reroll_dice.push(roll);

			if roll == 5 || roll == 6 {
				hits += 1;
			}
		}
	}

	RollResult {
		dice: dice,
		reroll_dice: reroll_dice,
		hits: hits,
		glitch: glitch,
		roll_type: roll_type
	}
}
