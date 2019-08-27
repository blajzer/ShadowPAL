extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Metatype {
	Human,
	Elf,
	Dwarf,
	Ork,
	Troll
}

impl std::fmt::Display for Metatype {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
			match &self {
				Metatype::Human => "Human",
				Metatype::Elf => "Elf",
				Metatype::Dwarf => "Dwarf",
				Metatype::Ork => "Ork",
				Metatype::Troll => "Troll"
			}
		)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Archetype {
	Adept,
	Decker,
	Face,
	Mage,
	Rigger,
	Shaman,
	StreetSamurai,
	Technomancer
}

impl std::fmt::Display for Archetype {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
			match &self {
				Archetype::Adept => "Adept",
				Archetype::Decker => "Decker",
				Archetype::Face => "Face",
				Archetype::Mage => "Mage",
				Archetype::Rigger => "Rigger",
				Archetype::Shaman => "Shaman",
				Archetype::StreetSamurai => "Street Samurai",
				Archetype::Technomancer => "Technomancer"
			}
		)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
	pub body: u8,
	pub agility: u8,
	pub reaction: u8,
	pub strength: u8,
	pub will: u8,
	pub logic: u8,
	pub intuition: u8,
	pub charisma: u8,
	pub edge: u8,
	pub magic_or_resonance: u8,
	pub essence: f32,
	pub physical_damage: u8,
	pub stun_damage: u8,
	pub name: String,
	pub metatype: Metatype,
	pub archetype: Archetype
}

impl Character {
	pub fn mental_limit(&self) -> usize {
		let inner = (self.logic * 2 + self.intuition + self.will) as f32;
		(inner / 3.0 + 0.5).floor() as usize
	}

	pub fn physical_limit(&self) -> usize {
		let inner = (self.strength * 2 + self.body + self.reaction) as f32;
		(inner / 3.0 + 0.5).floor() as usize
	}

	pub fn social_limit(&self) -> usize {
		let inner = (self.charisma * 2 + self.will) as f32 + self.essence;
		(inner / 3.0 + 0.5).floor() as usize
	}

	pub fn physical_damage_max(&self) -> usize {
		(self.body as usize) / 2 + 8
	}

	pub fn stun_damage_max(&self) -> usize {
		(self.will as usize) / 2 + 8
	}
}