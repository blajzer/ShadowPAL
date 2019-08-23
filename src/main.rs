#![windows_subsystem = "windows"]

extern crate gtk;
extern crate gio;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder};

use std::env::args;

mod character;
mod dice;

fn make_character_widget(char_data: &character::Character) -> gtk::Builder {
	let glade_src = include_str!("ui/character.ui");
	let builder = Builder::new_from_string(glade_src);

	let name_archetype_label: gtk::Label = builder.get_object("NameArchetypeLabel").expect("Couldn't get NameArchetypeLabel");

	let name_archetype_text = format!("<span font_weight=\"heavy\" size=\"18432\">{}</span><span size=\"20480\"> - </span><span size=\"16384\" font_style=\"italic\">{} {}</span>",
		char_data.name,
		char_data.metatype,
		char_data.archetype
	);
	name_archetype_label.set_markup(name_archetype_text.as_str());

	builder
}

fn build_ui(application: &gtk::Application) {
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
		let test_data = character::Character {
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
			name: "Beefy Guard".to_string(),
			metatype: character::Metatype::Troll,
			archetype: character::Archetype::StreetSamurai
		};

		let npc = make_character_widget(&test_data);
		let npc_root: gtk::Frame = npc.get_object("root").expect("Couldn't get root");
		npc_list.add(&npc_root);
	}

	window.show_all();
}

fn main() {
	let application =
		gtk::Application::new(Some("com.blajzer.shadowpal"), Default::default())
		.expect("Initialization failed...");

	application.connect_activate(|app| { build_ui(app); });

    application.run(&args().collect::<Vec<_>>());
}
