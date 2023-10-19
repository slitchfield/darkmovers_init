use crate::entity;
use names::Generator;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: u32,
    #[serde(skip)]
    d6value: u32,
    #[serde(skip)]
    mookvalue: u32,
    #[serde(skip)]
    mookd6value: u32,

    entity_list: Vec<entity::Entity>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 10,
            d6value: 1,
            mookvalue: 10,
            mookd6value: 1,
            entity_list: vec![],
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            d6value,
            mookvalue,
            mookd6value,
            entity_list,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                    if ui.button("Clear Entity List").clicked() {
                        entity_list.clear();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.heading("Bottom Panel");

            ui.label("Name:");
            ui.text_edit_singleline(label);
            ui.label("Base Init.:");
            ui.add(egui::Slider::new(value, 0..=20));
            ui.label("Init d6");
            ui.add(egui::Slider::new(d6value, 0..=6));

            if ui.button("Add Entity").clicked() {
                let entity = entity::Entity::new(label, *value, *d6value);
                println!("Adding entity: {}", entity);
                entity_list.push(entity);
            }
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Predefined Mooks");

            ui.strong("Profile A (6+1d6)");
            if ui.button("Generate Mook").clicked() {
                let mut generator = Generator::default();
                let random_name = format!("Mook {}", generator.next().unwrap());
                let entity = entity::Entity::new(random_name.as_str(), 6, 1);
                entity_list.push(entity);
            }
            ui.strong("Profile B (9+1d6)");
            if ui.button("Generate Mook").clicked() {
                let mut generator = Generator::default();
                let random_name = format!("Mook {}", generator.next().unwrap());
                let entity = entity::Entity::new(random_name.as_str(), 6, 1);
                entity_list.push(entity);
            }
            ui.strong("Other");
            ui.label("Base Init.:");
            ui.add(egui::Slider::new(mookvalue, 0..=20));
            ui.label("Init d6");
            ui.add(egui::Slider::new(mookd6value, 0..=6));
            if ui.button("Generate Mook").clicked() {
                let mut generator = Generator::default();
                let random_name = format!("Mook {}", generator.next().unwrap());
                let entity = entity::Entity::new(random_name.as_str(), *mookvalue, *mookd6value);
                entity_list.push(entity);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Bulk Actions");
            if ui.button("Reroll all initiatives").clicked() {
                for entity in entity_list.iter_mut() {
                    entity.reroll_init();
                }
            }
            if ui.button("Reorder by initiative").clicked() {
                entity_list.sort_by(|a, b| b.cur_init.cmp(&a.cur_init));
            }
            if ui.button("Refresh combat pass").clicked() {
                for entity in entity_list.iter_mut() {
                    if entity.cur_init > 0 {
                        entity.turn_taken = false;
                    }
                }
            }

            ui.heading("Initiative Entities:");
            let mut remove_list: Vec<String> = vec![];
            for entity in entity_list.iter_mut() {
                ui.horizontal(|ui| {
                    if entity.turn_taken {
                        ui.label(format!("{}", entity));
                    } else {
                        ui.strong(format!("{}", entity));
                    }
                    if ui.button("Take Turn").clicked() {
                        entity.take_turn();
                    }
                    if ui.button("Interrupt (-5)").clicked() {
                        entity.interrupt(5);
                    }
                    if ui.button("Interrupt (-10)").clicked() {
                        entity.interrupt(10);
                    }
                    if ui.button("Increment Initiative (+1)").clicked() {
                        entity.cur_init += 1;
                    }
                    if ui.button("Remove").clicked() {
                        remove_list.push(String::from(&entity.name));
                    };
                });
            }
            entity_list.retain(|e| !remove_list.contains(&e.name));
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
