use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use egui_probe::{EguiProbe, Probe};

#[derive(Debug, Clone, EguiProbe, PartialEq, Default)]
pub struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, EguiProbe)]
pub struct Shared {
    /// this value will be shared between two threads, one will be updated by the ui
    value: Arc<RwLock<Person>>,
}
fn main() {
    let shared = Shared {
        value: Arc::new(RwLock::new(Person::default())),
    };
    let other = shared.clone();
    std::thread::spawn(move || {
        loop {
            {
                let r = other.value.read().unwrap();
                println!("value is: {r:#?}");
            }
            std::thread::sleep(Duration::from_secs(1));
            {
                other.value.write().unwrap().name += "a";
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-probe demo app",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiProbeDemoApp::new(cc, shared)))),
    )
    .unwrap();
}

struct EguiProbeDemoApp {
    value: Shared,
}

impl EguiProbeDemoApp {
    fn new(_cc: &eframe::CreationContext<'_>, shared: Shared) -> Self {
        EguiProbeDemoApp { value: shared }
    }
}

impl eframe::App for EguiProbeDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            egui::widgets::global_theme_preference_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                Probe::new(&mut self.value).show(ui);
            });
        });
    }
}
