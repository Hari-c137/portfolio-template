#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct MyApp {
    aboutme: bool,
    philosphy: bool,
    projects: bool,
    skills: bool,
    site: bool,
    links: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            aboutme: false,
            philosphy: false,
            projects: false,
            skills: false,
            site: false,
            links: false,
        }
    }
}

fn apply_gruvbox_dark(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.dark_mode = true;
    visuals.panel_fill = egui::Color32::from_rgb(40, 40, 40); // #282828
    visuals.window_fill = egui::Color32::from_rgb(50, 48, 47); // #32302f
    visuals.extreme_bg_color = egui::Color32::from_rgb(29, 32, 33); // #1d2021

    visuals.override_text_color = Some(egui::Color32::from_rgb(235, 219, 178)); // #ebdbb2

    ctx.set_visuals(visuals);
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        apply_gruvbox_dark(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::Panel::right("sidepanel_right")
            .resizable(true)
            .min_size(100.0)
            .max_size(200.0)
            .show(ui, |ui| {
                ui.heading("Hari-c137's about-rs");
                ui.separator();

                if ui.button("About Me").clicked() {
                    self.aboutme = true;
                }
                if ui.button("Philosphy").clicked() {
                    self.philosphy = true;
                }
                if ui.button("My Projects").clicked() {
                    self.projects = true;
                }
                if ui.button("Skills").clicked() {
                    self.skills = true;
                }
                if ui.button("Site").clicked() {
                    self.site = true;
                }
                if ui.button("Links").clicked() {
                    self.links = true;
                }
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(ui.available_height() / 2.3); // Push down vertically
                ui.heading("'Good design is as little design as possible.'");
                ui.heading("--some German motherfucker");
            });
        });

        egui::Window::new("About Me")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(egui::Pos2::new(300.0, 200.0))
            .open(&mut self.aboutme)
            .show(ui, |ui| {
                ui.label(
                    " Hey you..!   I'm clearlyopaque, but for those who already knows me and where they know me by, i also go by Harry or Hari-c137. i like to describe myself as an 20 year old self-taught (dys)functional programmer who loves cats.",
                );
            });
        egui::Window::new("Philosphy")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(egui::Pos2::new(500.0, 100.0))
            .open(&mut self.philosphy)
            .show(ui, |ui| {
                ui.label("
you can agree or or disagree to what i got say, everyone has freedom of speech

in my honest opinion normal people shouldn't be allowed to use computers in the first place, for the ease of use for inferior/dumber individuals, we are purposefully making complex systems that are meant to complex and smarter than us purposely dumber..for what! so people doomscroll social media while they watch +18 videos of people that looks <18? access to a computer and internet should be privilege and not something every retarded pedofiles on the world can get access to. 
                ");
            });
        egui::Window::new("My Projects")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(egui::Pos2::new(600.0, 400.0))
            .open(&mut self.projects)
            .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {

            ui.hyperlink_to("1. Echelon", "https://github.com/Hari-c137/echelon");
            ui.label(" is an experimental unstable highly-opinionated NixOS based custom multi-user infrastructure that i made for my personal desktop, laptops and server");
            ui.separator();
            ui.hyperlink_to("2. Slyfox", "https://github.com/Hari-c137/slyfox");
            ui.label("user.js and userchrome files for minimal firefox UI before zen and helium were a thing");
            ui.separator();
            ui.hyperlink_to("3. Clearwm", "https://github.com/Hari-c137/clearwm");
            ui.label("minimal window manager implementation for river wayland compositor written in rust.");
            ui.separator();
            ui.hyperlink_to("4. Startpage", "https://github.com/hari-c137.github.io");
            ui.label("a minimal,keyboard-driven startpage with smart suggestions for power users and lazy developers");
            ui.separator();
            ui.hyperlink_to("5. c137-wm", "https://github.com/Hari-c137/c137-wm");
            ui.label("my custom fork of dwl wayland compositor with patches");
            ui.separator();
            ui.hyperlink_to("6. SignlangInterpreter", "https://github.com/Hari-c137/signlanginterpreter");
            ui.label("language standard agnostic sign language interpreter");
            ui.separator();
            ui.hyperlink_to("7. brainfuck", "https:://codeberg.org/clearlyopaque/brainfuck");
            ui.label("brainfuck interpreter implementation in nim(complete) and rust(rewrite)");
            ui.separator();
            ui.hyperlink_to("8. wireframe", "https:://codeberg.org/clearlyopaque/wireframe");
            ui.label("wireframe rendering of 3D primitive shapes");
            ui.separator();
            ui.hyperlink_to("9. discord bypasser", "https://github.com/Hari-c137/.well-known");
            ui.label("bypasser for discord custom domain verification through HTTPS (guide)");
            ui.separator();
            ui.hyperlink_to("10. Opensource Contributions and More..", "https:://github.com/Hari-c137");
            ui.label("active open source contributor and maintainer for multiple orgainzations such as voidlinux.org, nixpkgs, niri-wayland compositor, zed-editor, cachyos, dxvk, Linux-kernel's xpad driver etc etc.. ");
            ui.separator();
            ui.label("and other multiple softwares that i wrote that will be soon opensourced and opensource ones that am too lazy to list out here.. just go fucking look at my github.");
            ui.separator();
           });
            });
        egui::Window::new("My Skills")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(egui::Pos2::new(100.0, 600.0))
            .open(&mut self.skills)
            .show(ui, |ui| {
                ui.label("
 area of interest: systems programming, operating system development, unix-based sytems, kernel developement, graphics programming,eBPF technology, distributed systems, compilers, shader programming, functional programming, emulation, virtualization, containerization, shell-scripting,

 programming langauges: rust, C, nim, haskell, nix, bash-scripting

 system: {

    for most of my low level programming:  i use a potatop with 1-core 2 thread CPU with 4gb ram

    for research and experimental work:    i use my custom build BigBlackComputer (BBC) with i5-1400F, RX 7600 8GB, 64GB RAM AND 4TB STORAGE

}

       ")
            });
        egui::Window::new("About site")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(egui::Pos2::new(900.0, 500.0))
            .open(&mut self.site)
            .show(ui, |ui| {
                ui.label( "
if you have noticed anything weird or out of place about this website.. then yes you are correct.. first things first.. this website is entirely hand-written by me, that means no 'vibe coding' or no use of AI, MCP, agents, harness or any kinds of that bullshit therefore if(when) u find bugs, incosistences or a spelling mistake(cause i cant spell for shit) you can 100% blame me and not some fucking AI. most of the 'portfolios' nowadays are AIslop that a LLM vomits out for people to blindly stare at.. so here is something new for change.. a website made by human for humans. we are humans and we humans make mistakes..deal with it.

another reason this website is looking kinda weird is that, as u could tell am not a webdev its not that i dont know webdev.. its cause i chose not to indulge in mediocre programming but when i do do it once in a blue moon i make better websites than most AIslop anyways.., i mostly do systems programming and other low level stuff, and with no interest in touching webdev in ever.. BEHOLD.. a website written in rust using the egui library which compiles to webassembly using trunk wasm builder.

also this site is still in active developement as am building this website as my future actual portfolio.. not this dummy one i made..therefore the underlying architecture of this website might not change but i cant promise that less uncertainty to the content of website ");
});
        egui::Window::new("Links")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(egui::Pos2::new(500.0, 800.0))
            .open(&mut self.links)
            .show(ui, |ui| {
                ui.label("Contact me:");
                ui.separator();
                link(ui);
            });
    }
}

fn link(ui: &mut egui::Ui) {
    use egui::special_emojis::GITHUB;
    ui.hyperlink_to(
        format!("{GITHUB} Hari-c137"),
        "https://github.com/Hari-c137",
    );
    ui.hyperlink_to(
        "Codeberg (github is shit)",
        "https://codeberg.org/clearlyopaque",
    );
    ui.hyperlink_to(
        "Startpage (only the worthy are allowed)",
        "https://hari-c137.github.io",
    );
    ui.hyperlink_to(
        "Blog (notes,infodump and rant)",
        "https://hari-c137-pages.dev",
    );
}

// background
// shaders {vertex and shaders}
// conways game of life
// blob inbuilt compiler, os?!
