// TODO: re-enable for ship... is there a way to make this debug-only?
//#![windows_subsystem = "windows"]

extern crate gtk;
extern crate gio;
extern crate ron;
extern crate serde;

use serde::{Deserialize, Serialize};

use gio::prelude::*;
use gtk::prelude::*;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::{ApplicationWindow, Builder};

use std::env::args;

mod character;
mod dice;

#[derive(Serialize, Deserialize, Debug)]
struct DataStore {
	active_characters: Vec<Rc<RefCell<character::Character>>>,
	template_characters: Vec<character::Character>
}

fn make_character_widget(char_data: Rc<RefCell<character::Character>>) -> gtk::Builder {
	let glade_src = include_str!("ui/character.ui");
	let builder = Builder::new_from_string(glade_src);

	let char_data_ref = char_data.borrow();

	// Name
	let name_label: gtk::Label = builder.get_object("NameLabel").expect("Couldn't get NameLabel");
	let name_text = format!("<span font_weight=\"heavy\" size=\"18432\">{}</span>",
		char_data_ref.name
	);
	name_label.set_markup(name_text.as_str());

	// Metatype and archetype
	let archetype_label: gtk::Label = builder.get_object("ArchetypeLabel").expect("Couldn't get ArchetypeLabel");
	let archetype_text = format!("<span size=\"16384\" font_style=\"italic\">{} {}</span>",
		char_data_ref.metatype,
		char_data_ref.archetype
	);
	archetype_label.set_markup(archetype_text.as_str());

	// Stats
	let body: gtk::Label = builder.get_object("BOD").expect("Couldn't get BOD");
	body.set_text(char_data_ref.body.to_string().as_str());

	let agility: gtk::Label = builder.get_object("AGI").expect("Couldn't get AGI");
	agility.set_text(char_data_ref.agility.to_string().as_str());

	let reaction: gtk::Label = builder.get_object("REA").expect("Couldn't get REA");
	reaction.set_text(char_data_ref.reaction.to_string().as_str());

	let strength: gtk::Label = builder.get_object("STR").expect("Couldn't get STR");
	strength.set_text(char_data_ref.strength.to_string().as_str());

	let will: gtk::Label = builder.get_object("WIL").expect("Couldn't get WIL");
	will.set_text(char_data_ref.will.to_string().as_str());

	let logic: gtk::Label = builder.get_object("LOG").expect("Couldn't get LOG");
	logic.set_text(char_data_ref.logic.to_string().as_str());

	let intuition: gtk::Label = builder.get_object("INT").expect("Couldn't get INT");
	intuition.set_text(char_data_ref.intuition.to_string().as_str());

	let charisma: gtk::Label = builder.get_object("CHA").expect("Couldn't get CHA");
	charisma.set_text(char_data_ref.charisma.to_string().as_str());

	let edge: gtk::Label = builder.get_object("EDG").expect("Couldn't get EDG");
	edge.set_text(char_data_ref.edge.to_string().as_str());

	let magic_or_resonance: gtk::Label = builder.get_object("MAG").expect("Couldn't get MAG");
	magic_or_resonance.set_text(char_data_ref.magic_or_resonance.to_string().as_str());

	let magic_or_resonance_header: gtk::Label = builder.get_object("MagOrRes").expect("Couldn't get MagOrRes");
	if char_data_ref.archetype == character::Archetype::Technomancer {
		magic_or_resonance_header.set_text("RES");
	}

	// Damage tracks
	// Physical damage
	{
		let damage_grid: gtk::Grid = builder.get_object("PhysicalGrid").expect("Couldn't get PhysicalGrid");

		// remove all placeholder widgets
		damage_grid.foreach(|c| { c.destroy(); });

		// Make new widgets
		for i in 0..char_data_ref.physical_damage_max() {
			let data_ref = char_data.clone();
			let checkbox = gtk::CheckButtonBuilder::new()
				.active(i < char_data_ref.physical_damage as usize)
				.build();

			checkbox.connect_clicked(move |c| {
				// HACK: Gross hack to prevent us from answering the activate callback multiple times.
				// Only one of these can/is expected to run at a time. We use the mutable borrow
				// detection to determine if there's another running already.
				let mut mut_char = if let Ok(inner) = data_ref.try_borrow_mut() {
					inner
				} else {
					return;
				};

				mut_char.physical_damage = i as u8 + if c.get_active() { 1 } else { 0 };
				
				if let Some(parent_widget) = c.get_parent() {
					if let Ok(parent) = gtk::Cast::dynamic_cast::<gtk::Grid>(parent_widget) {
						for j in 0..mut_char.physical_damage_max() {
							let row = (j / 3) as i32;
							let col = (j % 3) as i32;

							if let Some(check_widget) = parent.get_child_at(col, row) {
								if let Ok(check) = gtk::Cast::dynamic_cast::<gtk::CheckButton>(check_widget) {
									if j < i {
										check.set_active(true);
									} else if j > i {
										check.set_active(false);
									}
								}
							}
						}
					}
				}
			});

			let cur_row = (i / 3) as i32;
			let cur_col = (i % 3) as i32;
			damage_grid.attach(&checkbox, cur_col, cur_row, 1, 1);
		}
	}

	// Stun damage
	// TODO: dedup
	{
		let damage_grid: gtk::Grid = builder.get_object("StunGrid").expect("Couldn't get StunGrid");

		// remove all placeholder widgets
		damage_grid.foreach(|c| { c.destroy(); });

		// Make new widgets
		for i in 0..char_data_ref.stun_damage_max() {
			let data_ref = char_data.clone();
			let checkbox = gtk::CheckButtonBuilder::new()
				.active(i < char_data_ref.stun_damage as usize)
				.build();

			checkbox.connect_clicked(move |c| {
				// HACK: Gross hack to prevent us from answering the activate callback multiple times.
				// Only one of these can/is expected to run at a time. We use the mutable borrow
				// detection to determine if there's another running already.
				let mut mut_char = if let Ok(inner) = data_ref.try_borrow_mut() {
					inner
				} else {
					return;
				};

				mut_char.stun_damage = i as u8 + if c.get_active() { 1 } else { 0 };

				if let Some(parent_widget) = c.get_parent() {
					if let Ok(parent) = gtk::Cast::dynamic_cast::<gtk::Grid>(parent_widget) {
						for j in 0..mut_char.stun_damage_max() {
							let row = (j / 3) as i32;
							let col = (j % 3) as i32;

							if let Some(check_widget) = parent.get_child_at(col, row) {
								if let Ok(check) = gtk::Cast::dynamic_cast::<gtk::CheckButton>(check_widget) {
									if j < i {
										check.set_active(true);
									} else if j > i {
										check.set_active(false);
									}
								}
							}
						}
					}
				}
			});

			let cur_row = (i / 3) as i32;
			let cur_col = (i % 3) as i32;
			damage_grid.attach(&checkbox, cur_col, cur_row, 1, 1);
		}
	}

	builder
}

fn build_ui(application: &gtk::Application, gds: Rc<RefCell<DataStore>>) {
	let glade_src = include_str!("ui/main.ui");

	let builder = Builder::new_from_string(glade_src);
	let window: ApplicationWindow = builder.get_object("MainWindow").expect("Couldn't get MainWindow");
	window.set_application(Some(application));

	let roll_dice_button: gtk::Button = builder.get_object("RollDice").expect("Couldn't get RollDice");
	let roll_dice_count: gtk::SpinButton = builder.get_object("DiceCount").expect("Couldn't get DiceCount");
	let roll_type: gtk::ComboBoxText = builder.get_object("RollType").expect("Couldn't get RollType");
	let dice_output: gtk::TextBuffer = builder.get_object("DiceOutput").expect("Couldn't get DiceOutput");
	let hit_label: gtk::Label = builder.get_object("HitLabel").expect("Couldn't get HitLabel");
	let glitch_label: gtk::Label = builder.get_object("GlitchLabel").expect("Couldn't get GlitchLabel");

	roll_dice_button.connect_clicked(move |_| {
		let dice_count = roll_dice_count.get_value_as_int() as usize;
		let roll_type_enum = if let Some(s) = roll_type.get_active_text() {
			match s.as_str() {
				"Standard" => dice::RollType::Standard,
				"Push The Limit" => dice::RollType::ReRollSixes,
				"Second Chance" => dice::RollType::ReRollMisses,
				_ => dice::RollType::Standard
			}
		} else {
			dice::RollType::Standard
		};

		let roll_result = dice::roll(dice_count, roll_type_enum);

		// Set hits
		let mut new_hit_text = "<b>Hits:</b> ".to_string();
		new_hit_text.push_str(roll_result.hits.to_string().as_str());
		hit_label.set_markup(new_hit_text.as_str());

		// Set glitch
		let mut new_glitch_text = "<b>Glitch:</b> ".to_string();
		new_glitch_text.push_str(
			match roll_result.glitch {
				dice::Glitch::None => "No",
				dice::Glitch::Glitch => "Yes",
				dice::Glitch::CriticalGlitch => "Crit"
			}	
		);
		glitch_label.set_markup(new_glitch_text.as_str());

		// Roll results
		let mut result_text = String::new();
		for &d in roll_result.dice.iter() {
			result_text.push_str(d.to_string().as_str());
			result_text.push('\n');
		}
		
		if roll_result.reroll_dice.len() > 0 {
			result_text.push_str("\nRe-rolled dice:\n");
			for &d in roll_result.reroll_dice.iter() {
				result_text.push_str(d.to_string().as_str());
				result_text.push('\n');
			}
		}

		dice_output.set_text(result_text.as_str());
	});

	let npc_list: gtk::Box = builder.get_object("NPCList").expect("Couldn't get NPCList");
	{
		for character in gds.borrow().active_characters.iter() {
			let npc = make_character_widget(character.clone());
			let npc_root: gtk::Frame = npc.get_object("root").expect("Couldn't get root");
			npc_list.add(&npc_root);
		}
	}

	window.show_all();
}

fn main() {
	// De-serialize or create a new one
	let gds = Rc::new(RefCell::new(if let Ok(contents) = fs::read_to_string("shadowpal.db") {
		if let Ok(deserialized) = ron::de::from_str(contents.as_str()) {
			deserialized
		} else {
			DataStore {
				active_characters: Vec::new(),
				template_characters: Vec::new()
			}
		}
	} else {
		DataStore {
			active_characters: Vec::new(),
			template_characters: Vec::new()
		}
	}));

	// XXX: test data
	{
		let test_data = Rc::new(RefCell::new(character::Character {
			body: 5,
			agility: 2,
			reaction: 6,
			strength: 6,
			will: 2,
			logic: 2,
			intuition: 3,
			charisma: 2,
			edge: 2,
			magic_or_resonance: 0,
			essence: 6.0,
			physical_damage: 0,
			stun_damage: 0,
			name: "Beefy Guard".to_string(),
			metatype: character::Metatype::Troll,
			archetype: character::Archetype::StreetSamurai
		}));
		gds.borrow_mut().active_characters.push(test_data);
	}

	let application =
		gtk::Application::new(Some("com.blajzer.shadowpal"), Default::default())
		.expect("Initialization failed...");

	let gds_copy = gds.clone();
	application.connect_activate(move |app| { build_ui(app, gds_copy.clone()); });

    application.run(&args().collect::<Vec<_>>());

	// Serialize 
	if let Ok(s) = ron::ser::to_string(&(*gds)) {
		let _ = fs::write("shadowpal.db", s);
	} else {
		println!("Couldn't write database.");
	}
}
