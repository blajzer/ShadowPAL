// TODO: re-enable for ship... is there a way to make this debug-only?
//#![windows_subsystem = "windows"]

extern crate gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};

extern crate gio;
use gio::prelude::*;

extern crate ron;
extern crate serde;
use serde::{Deserialize, Serialize};

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

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

	// Limits
	let physical_limit: gtk::Label = builder.get_object("PhysicalLimit").expect("Couldn't get PhysicalLimit");
	let physical_limit_text = format!("<b>Physical: </b>{}", char_data_ref.physical_limit());
	physical_limit.set_markup(physical_limit_text.as_str());

	let mental_limit: gtk::Label = builder.get_object("MentalLimit").expect("Couldn't get MentalLimit");
	let mental_limit_text = format!("<b>Mental: </b>{}", char_data_ref.mental_limit());
	mental_limit.set_markup(mental_limit_text.as_str());

	let social_limit: gtk::Label = builder.get_object("SocialLimit").expect("Couldn't get SocialLimit");
	let social_limit_text = format!("<b>Social: </b>{}", char_data_ref.social_limit());
	social_limit.set_markup(social_limit_text.as_str());

	// Initiative
	// TODO: dedup
	{
		let data_ref = char_data.clone();
		let initiative: gtk::Label = builder.get_object("Initiative").expect("Couldn't get Initiative");
		let initiative_copy = initiative.clone();
		let roll_initiative: gtk::EventBox = builder.get_object("RollInitiative").expect("Couldn't get RollInitiative");
		roll_initiative.connect_button_release_event(move |_, event| {
			if event.get_button() == 1 {
				let mut mut_char = data_ref.borrow_mut();
				mut_char.initiative = mut_char.reaction + mut_char.intuition + dice::basic_roll(1) as u8;

				let init_text = format!("<b>Initiative: </b>{}", mut_char.initiative);
				initiative.set_markup(init_text.as_str());

				gtk::Inhibit(false)
			} else {
				gtk::Inhibit(true)
			}
		});

		let data_ref = char_data.clone();
		let advance_initiative: gtk::EventBox = builder.get_object("AdvanceInitiative").expect("Couldn't get AdvanceInitiative");
		advance_initiative.connect_button_release_event(move |_, event| {
			if event.get_button() == 1 {
				let mut mut_char = data_ref.borrow_mut();
				mut_char.initiative = if mut_char.initiative >= 10 { mut_char.initiative - 10 } else { 0 };

				let init_text = format!("<b>Initiative: </b>{}", mut_char.initiative);
				initiative_copy.set_markup(init_text.as_str());

				gtk::Inhibit(false)
			} else {
				gtk::Inhibit(true)
			}
		});
	}

	{
		let data_ref = char_data.clone();
		let initiative: gtk::Label = builder.get_object("MatrixInitiative").expect("Couldn't get MatrixInitiative");
		let initiative_copy = initiative.clone();
		let roll_initiative: gtk::EventBox = builder.get_object("RollMatrixInitiative").expect("Couldn't get RollMatrixInitiative");
		let init_type: gtk::ComboBoxText = builder.get_object("MatrixInitType").expect("Couldn't get MatrixInitType");
		roll_initiative.connect_button_release_event(move |_, event| {
			if event.get_button() == 1 {
				let mut mut_char = data_ref.borrow_mut();

				mut_char.matrix_initiative = if let Some(s) = init_type.get_active_text() {
					match s.as_str() {
						"AR" => mut_char.reaction + mut_char.intuition + dice::basic_roll(1) as u8,
						"VR (Cold)" => mut_char.intuition + dice::basic_roll(3) as u8,
						"VR (Hot)" => mut_char.intuition + dice::basic_roll(4) as u8,
						_ => mut_char.reaction + mut_char.intuition + dice::basic_roll(1) as u8
					}
				} else {
					mut_char.reaction + mut_char.intuition + dice::basic_roll(1) as u8
				};

				let init_text = format!("<b>Matrix Init: </b>{}", mut_char.matrix_initiative);
				initiative.set_markup(init_text.as_str());

				gtk::Inhibit(false)
			} else {
				gtk::Inhibit(true)
			}
		});

		let data_ref = char_data.clone();
		let advance_initiative: gtk::EventBox = builder.get_object("AdvanceMatrixInitiative").expect("Couldn't get AdvanceMatrixInitiative");
		advance_initiative.connect_button_release_event(move |_, event| {
			if event.get_button() == 1 {
				let mut mut_char = data_ref.borrow_mut();
				mut_char.matrix_initiative = if mut_char.matrix_initiative >= 10 { mut_char.matrix_initiative - 10 } else { 0 };

				let init_text = format!("<b>Matrix Init: </b>{}", mut_char.matrix_initiative);
				initiative_copy.set_markup(init_text.as_str());

				gtk::Inhibit(false)
			} else {
				gtk::Inhibit(true)
			}
		});
	}

	{
		let data_ref = char_data.clone();
		let initiative: gtk::Label = builder.get_object("AstralInitiative").expect("Couldn't get AstralInitiative");
		let initiative_copy = initiative.clone();
		let roll_initiative: gtk::EventBox = builder.get_object("RollAstralInitiative").expect("Couldn't get RollAstralInitiative");
		roll_initiative.connect_button_release_event(move |_, event| {
			if event.get_button() == 1 {
				let mut mut_char = data_ref.borrow_mut();
				mut_char.astral_initiative = mut_char.intuition + mut_char.intuition + dice::basic_roll(2) as u8;

				let init_text = format!("<b>Astral Init: </b>{}", mut_char.astral_initiative);
				initiative.set_markup(init_text.as_str());

				gtk::Inhibit(false)
			} else {
				gtk::Inhibit(true)
			}
		});

		let data_ref = char_data.clone();
		let advance_initiative: gtk::EventBox = builder.get_object("AdvanceAstralInitiative").expect("Couldn't get AdvanceAstralInitiative");
		advance_initiative.connect_button_release_event(move |_, event| {
			if event.get_button() == 1 {
				let mut mut_char = data_ref.borrow_mut();
				mut_char.astral_initiative = if mut_char.astral_initiative >= 10 { mut_char.astral_initiative - 10 } else { 0 };

				let init_text = format!("<b>Astral Init: </b>{}", mut_char.astral_initiative);
				initiative_copy.set_markup(init_text.as_str());

				gtk::Inhibit(false)
			} else {
				gtk::Inhibit(true)
			}
		});
	}

	builder
}

fn rebuild_template_list(template_list: &gtk::ComboBoxText, templates: &Vec<character::Character>) {
	template_list.remove_all();

	let mut char_index = 0;
	for character in templates.iter() {
		template_list.append(Some(char_index.to_string().as_str()), character.name.as_str());
		
		char_index += 1;
	}
}

fn gtk_entry_to_num<T: std::str::FromStr>(entry: &gtk::Entry, default: T) -> T {
	if let Some(value_str) = entry.get_text() {
		if let Ok(value) = value_str.parse::<T>() {
			value
		} else {
			default
		}
	} else {
		default
	}
}

fn make_character_from_template(
	name: &gtk::Entry,
	metatype: &gtk::ComboBoxText,
	archetype: &gtk::ComboBoxText,
	body: &gtk::Entry,
	agility: &gtk::Entry,
	reaction: &gtk::Entry,
	strength: &gtk::Entry,
	will: &gtk::Entry,
	logic: &gtk::Entry,
	intuition: &gtk::Entry,
	charisma: &gtk::Entry,
	edge: &gtk::Entry,
	magic: &gtk::Entry,
	essence: &gtk::Entry
) -> character::Character {
	let name_string = if let Some(name_text) = name.get_text() {
		name_text.as_str().to_string()
	} else {
		"Unnamed Character".to_string()
	};

	character::Character {
		body: gtk_entry_to_num(&body, 0),
		agility: gtk_entry_to_num(&agility, 0),
		reaction: gtk_entry_to_num(&reaction, 0),
		strength: gtk_entry_to_num(&strength, 0),
		will: gtk_entry_to_num(&will, 0),
		logic: gtk_entry_to_num(&logic, 0),
		intuition: gtk_entry_to_num(&intuition, 0),
		charisma: gtk_entry_to_num(&charisma, 0),
		edge: gtk_entry_to_num(&edge, 0),
		magic_or_resonance: gtk_entry_to_num(&magic, 0),
		essence: gtk_entry_to_num(&essence, 6.0),
		physical_damage: 0,
		stun_damage: 0,
		initiative: 0,
		matrix_initiative: 0,
		astral_initiative: 0,
		name: name_string,
		metatype: if let Some(metatype_str) = metatype.get_active_text() {
			if let Ok(meta) = metatype_str.parse() {
				meta
			} else {
				character::Metatype::Human
			}
		} else {
			character::Metatype::Human
		},
		archetype: if let Some(archetype_str) = archetype.get_active_text() {
			if let Ok(arch) = archetype_str.parse() {
				arch
			} else {
				character::Archetype::Adept
			}
		} else {
			character::Archetype::Adept
		}
	}
}

fn add_character_to_ui(
	character: &Rc<RefCell<character::Character>>,
	gds: &Rc<RefCell<DataStore>>,
	npc_list: &gtk::Box
) {
	let npc = make_character_widget(character.clone());
	let npc_root: gtk::Frame = npc.get_object("root").expect("Couldn't get root");

	// Connect "delete" button
	let delete_npc: gtk::Button = npc.get_object("DeleteNPC").expect("Couldn't get DeleteNPC");

	let char_clone = character.clone();
	let gds_clone = gds.clone();
	let npc_root_copy = npc_root.clone();
	let npc_list_copy = npc_list.clone();

	delete_npc.connect_clicked(move |_| {
		let mut mut_gds = gds_clone.borrow_mut();

		if let Some(position) = mut_gds.active_characters.iter().position(|c| Rc::ptr_eq(c, &char_clone)) {
			mut_gds.active_characters.remove(position);
			npc_list_copy.remove(&npc_root_copy);
		}
	});

	npc_list.add(&npc_root);
}

fn build_ui(application: &gtk::Application, gds: Rc<RefCell<DataStore>>) {
	let glade_src = include_str!("ui/main.ui");

	let builder = Builder::new_from_string(glade_src);
	let window: ApplicationWindow = builder.get_object("MainWindow").expect("Couldn't get MainWindow");
	window.set_application(Some(application));

	// Dice rolling section
	{
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
	}

	// Active NPC List
	let npc_list: gtk::Box = builder.get_object("NPCList").expect("Couldn't get NPCList");
	{
		for character in gds.borrow().active_characters.iter() {
			add_character_to_ui(character, &gds, &npc_list);
		}
	}

	// NPC Templates
	{
		let template_instantiate: gtk::Button = builder.get_object("TemplateInstantiate").expect("Couldn't get TemplateInstantiate");
		let template_save: gtk::Button = builder.get_object("TemplateSave").expect("Couldn't get TemplateSave");
		let template_delete: gtk::Button = builder.get_object("TemplateDelete").expect("Couldn't get TemplateDelete");
		let template_list: gtk::ComboBoxText = builder.get_object("TemplateList").expect("Couldn't get TemplateList");

		let template_name: gtk::Entry = builder.get_object("TName").expect("Couldn't get TName");
		let template_metatype: gtk::ComboBoxText = builder.get_object("TMetatype").expect("Couldn't get TMetatype");
		let template_archetype: gtk::ComboBoxText = builder.get_object("TArchetype").expect("Couldn't get TArchetype");
		let tbod: gtk::Entry = builder.get_object("TBOD").expect("Couldn't get TBOD");
		let tagi: gtk::Entry = builder.get_object("TAGI").expect("Couldn't get TAGI");
		let trea: gtk::Entry = builder.get_object("TREA").expect("Couldn't get TREA");
		let tstr: gtk::Entry = builder.get_object("TSTR").expect("Couldn't get TSTR");
		let twil: gtk::Entry = builder.get_object("TWIL").expect("Couldn't get TWIL");
		let tlog: gtk::Entry = builder.get_object("TLOG").expect("Couldn't get TLOG");
		let tint: gtk::Entry = builder.get_object("TINT").expect("Couldn't get TINT");
		let tcha: gtk::Entry = builder.get_object("TCHA").expect("Couldn't get TCHA");
		let tedg: gtk::Entry = builder.get_object("TEDG").expect("Couldn't get TEDG");
		let tmag: gtk::Entry = builder.get_object("TMAG").expect("Couldn't get TMAG");
		let tess: gtk::Entry = builder.get_object("TESS").expect("Couldn't get TESS");

		// initialize the template list
		rebuild_template_list(&template_list, &gds.borrow().template_characters);

		// Delete button
		let gds_clone = gds.clone();
		let template_list_clone = template_list.clone();
		template_delete.connect_clicked(move |_| {
			if let Some(id_str) = template_list_clone.get_active_id() {
				if let Ok(id) = id_str.as_str().parse::<i32>() {
					if id >= 0 && id < (gds_clone.borrow().template_characters.len() as i32){
						let mut mut_gds = gds_clone.borrow_mut();

						mut_gds.template_characters.remove(id as usize);
						rebuild_template_list(&template_list_clone, &mut_gds.template_characters);
					}
				}
			}
		});

		// Save button
		let gds_clone = gds.clone();
		let template_list_clone = template_list.clone();
		let template_name_clone = template_name.clone();
		let template_metatype_clone = template_metatype.clone();
		let template_archetype_clone = template_archetype.clone();
		let tbod_clone = tbod.clone();
		let tagi_clone = tagi.clone();
		let trea_clone = trea.clone();
		let tstr_clone = tstr.clone();
		let twil_clone = twil.clone();
		let tlog_clone = tlog.clone();
		let tint_clone = tint.clone();
		let tcha_clone = tcha.clone();
		let tedg_clone = tedg.clone();
		let tmag_clone = tmag.clone();
		let tess_clone = tess.clone();
		let window_clone = window.clone();
		template_save.connect_clicked(move |_| {
			let mut mut_gds = gds_clone.borrow_mut();

			let character = make_character_from_template(
				&template_name_clone,
				&template_metatype_clone,
				&template_archetype_clone,
				&tbod_clone,
				&tagi_clone,
				&trea_clone,
				&tstr_clone,
				&twil_clone,
				&tlog_clone,
				&tint_clone,
				&tcha_clone,
				&tedg_clone,
				&tmag_clone,
				&tess_clone);

			if let Some(pos) = mut_gds.template_characters.iter().position(|c| c.name == character.name) {
				let dialog = gtk::Dialog::new_with_buttons(
						Some("Replace Existing Template?"),
						Some(&window_clone),
						gtk::DialogFlags::MODAL,
						&[("Don't Save", gtk::ResponseType::Reject),
						("Replace Existing", gtk::ResponseType::Accept)]);
					
					match dialog.run() {
						gtk::ResponseType::Accept => {
							mut_gds.template_characters[pos] = character;
						},
						_ => ()
					}

					dialog.destroy();
			} else {
				mut_gds.template_characters.push(character);
				rebuild_template_list(&template_list_clone, &mut_gds.template_characters);
				template_list_clone.set_active_id(Some((mut_gds.template_characters.len() - 1).to_string().as_str()));
			}
		});


		// Instantiate button
		let gds_clone = gds.clone();
		let template_name_clone = template_name.clone();
		let template_metatype_clone = template_metatype.clone();
		let template_archetype_clone = template_archetype.clone();
		let tbod_clone = tbod.clone();
		let tagi_clone = tagi.clone();
		let trea_clone = trea.clone();
		let tstr_clone = tstr.clone();
		let twil_clone = twil.clone();
		let tlog_clone = tlog.clone();
		let tint_clone = tint.clone();
		let tcha_clone = tcha.clone();
		let tedg_clone = tedg.clone();
		let tmag_clone = tmag.clone();
		let tess_clone = tess.clone();
		template_instantiate.connect_clicked(move |_| {
			let mut mut_gds = gds_clone.borrow_mut();

			let character = Rc::new(RefCell::new(make_character_from_template(
				&template_name_clone,
				&template_metatype_clone,
				&template_archetype_clone,
				&tbod_clone,
				&tagi_clone,
				&trea_clone,
				&tstr_clone,
				&twil_clone,
				&tlog_clone,
				&tint_clone,
				&tcha_clone,
				&tedg_clone,
				&tmag_clone,
				&tess_clone)));

			mut_gds.active_characters.push(character.clone());
			add_character_to_ui(&character, &gds_clone, &npc_list);
		});
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
			initiative: 0,
			matrix_initiative: 0,
			astral_initiative: 0,
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
