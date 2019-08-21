#[derive(Debug)]
pub enum Metatype {
	Human,
	Elf,
	Dwarf,
	Ork,
	Troll
}

#[derive(Debug)]
pub struct Character {
	body: u8,
	agility: u8,
	reaction: u8,
	strength: u8,
	will: u8,
	logic: u8,
	intuition: u8,
	charisma: u8,
	edge: u8,
	magic_or_resonance: u8,
	essence: f32,
	metatype: Metatype
}

impl Character {
	pub fn mental_limit(&self) -> u8 {
		let inner = (self.logic * 2 + self.intuition + self.will) as f32;
		(inner / 3.0 + 0.5).floor() as u8
	}

	pub fn physical_limit(&self) -> u8 {
		let inner = (self.strength * 2 + self.body + self.reaction) as f32;
		(inner / 3.0 + 0.5).floor() as u8
	}

	pub fn social_limit(&self) -> u8 {
		let inner = (self.charisma * 2 + self.will) as f32 + self.essence;
		(inner / 3.0 + 0.5).floor() as u8
	}

	
}