#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    label: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

// pub struct Aboutrs {
//     name: String,
//     value: f32,
// }

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello Hell...!!!".to_owned(),
            value: 13.7,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // top panel
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // right panel
        egui::Panel::right("sidepanel_right").show(ui, |ui| {
            powered_by_egui_and_eframe(ui);
            if ui.button("button").clicked() {
                egui::Window::new("check").show(ui, |ui| {
                    ui.heading("hello");
                });
            };
            ui.heading("aboutrs");
        });

        egui::Window::new("window_1").show(ui, |ui| {
            ui.heading("buzz");
        });

        // bottom panel
        egui::Panel::bottom("bottom_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // Central Panel (main)
        egui::CentralPanel::default().show(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("busy with academia and research will finish the website q1 nov :/ ");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=137.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            // ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/Hari-c137/portfolio-template",
                "dare look at source code?"
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

            shitshit(ui);
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn shitshit(ui: &mut egui::Ui) {
    egui::Window::new("window1")
        .default_open(false)
        .show(ui, |ui| {
            ui.heading("hey");
        });
}

// todo
// [x] create side panel
// [ ] background of centerpanel
// [ ] create windows and buttons pointing to windows on side panel
// [ ] create windows::{aboutrs, disclamer, techstack, terminal, projects}
