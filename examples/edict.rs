//! This example shows how to control entities in the ECS world using egui-probe.

use edict::{
    component::{Component, Value},
    query::QueryBorrowAll,
    world::World,
    Entities, EntityId,
};
use egui_probe::EguiProbe;

trait Inspect: EguiProbe + Value {
    fn inspect(&mut self, ui: &mut egui::Ui);
}

impl<T> Inspect for T
where
    T: EguiProbe + Value,
{
    fn inspect(&mut self, ui: &mut egui::Ui) {
        egui_probe::Probe::new(self.name(), self).show(ui);
    }
}

#[derive(Component, EguiProbe, Default)]
#[edict(borrow(dyn Inspect))]
struct Number {
    value: u32,
}

#[derive(Component, EguiProbe, Default)]
#[edict(borrow(dyn Inspect))]
struct Text {
    text: String,
}

macro_rules! entity_component {
    ($c:ident in $e:ident; $world:ident, $ui:ident) => {
        let s = $e.has_component::<$c>();
        if $ui.selectable_label(s, stringify!($c)).clicked() {
            if s {
                $e.remove::<$c>();
            } else {
                let id = $e.id();
                $e.insert::<$c>($c::default());
                $e = $world.entity(id).unwrap();
            }
        }
    };
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-probe demo app",
        native_options,
        Box::new(|cc| Box::new(EguiProbeEdictApp::new(cc))),
    )
    .unwrap();
}

struct EguiProbeEdictApp {
    world: World,
    selected: Option<EntityId>,
}

impl EguiProbeEdictApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        EguiProbeEdictApp {
            world: World::new(),
            selected: None,
        }
    }
}

impl eframe::App for EguiProbeEdictApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { world, selected } = self;

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::SidePanel::left("entities").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.small_button("+").clicked() {
                    world.spawn_empty();
                }

                let mut entities = world
                    .view::<Entities>()
                    .iter()
                    .map(|e| e.id())
                    .collect::<Vec<_>>();

                entities.sort();

                for e in entities {
                    ui.horizontal(|ui| {
                        if ui.weak(format!("{e}")).clicked() {
                            *selected = Some(e);
                        }
                        let mut e = world.entity(e).unwrap();
                        entity_component!(Number in e; world, ui);
                        entity_component!(Text in e; world, ui);

                        if ui.small_button("-").clicked() {
                            if *selected == Some(e.id()) {
                                *selected = None;
                            }
                            e.despawn();
                        }
                    });
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut entities = world
                .view::<Entities>()
                .iter()
                .map(|e| e.id())
                .collect::<Vec<_>>();

            entities.sort();

            ui.vertical(|ui| {
                let mut cbox = egui::ComboBox::from_id_source("selected-entity");
                if let Some(e) = *selected {
                    if !entities.contains(&e) {
                        *selected = None;
                    } else {
                        cbox = cbox.selected_text(format!("{e}"));
                    }
                }

                cbox.show_ui(ui, |ui| {
                    for e in entities {
                        if ui
                            .selectable_label(*selected == Some(e), format!("{e}"))
                            .clicked()
                        {
                            *selected = Some(e);
                        }
                    }
                });

                if let Some(e) = *selected {
                    let e = world.entity(e).unwrap();
                    ui.vertical(|ui| {
                        let mut view = e.view_one::<QueryBorrowAll<&mut (dyn Inspect + Send)>>();
                        if let Some(components) = view.get_mut() {
                            for c in components {
                                ui.separator();
                                c.inspect(ui);
                            }
                        }
                    });
                }
            });
        });
    }
}
