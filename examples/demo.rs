use std::collections::HashMap;

use egui_probe::{angle, Probe};
use egui_probe_proc::EguiProbe;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-probe demo app",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiProbeDemoApp::new(cc)))),
    )
    .unwrap();
}

struct Foo;

fn custom_probe(_: &mut Foo, ui: &mut egui::Ui, _: &egui_probe::Style) -> egui::Response {
    ui.label("This is custom probe")
}

#[derive(EguiProbe, Clone)]
#[egui_probe(transparent)]
struct UpTo7(#[egui_probe(range = ..=7)] u32);

#[derive(EguiProbe)]
#[egui_probe(tags inlined)]
enum InlinedTags {
    Empty,

    #[egui_probe(transparent)]
    InlinedFloat(f32),

    Text {
        #[egui_probe(multiline)]
        text: String,
    },
}

#[derive(EguiProbe)]
#[egui_probe(tags combobox)]
enum ComboBoxTags {
    Empty,

    Num { value: usize },
}

impl Default for ComboBoxTags {
    fn default() -> Self {
        ComboBoxTags::Empty
    }
}

#[derive(Default, EguiProbe)]
#[egui_probe(rename_all = Train-Case)]
struct InnerValue {
    line: String,

    #[egui_probe(multiline)]
    multi_line: String,

    #[cfg(feature = "smallvec1")]
    small_vec_1: smallvec1::SmallVec<[String; 4]>,

    #[cfg(feature = "smallvec2")]
    small_vec_2: smallvec2::SmallVec<f32, 4>,

    #[cfg(feature = "hashbrown")]
    hash_brown: hashbrown::HashMap<u8, f32>,
}


#[derive(EguiProbe)]
struct DemoValue {
    boolean: bool,

    #[egui_probe(toggle_switch)]
    boolean_toggle: bool,

    float: f32,

    #[egui_probe(range = 22..=55)]
    range: usize,

    #[egui_probe(default=UpTo7(5))]
    maybe_range_to: Option<UpTo7>,

    #[egui_probe(range = 50..)]
    range_from: u8,

    #[egui_probe(as angle)]
    angle: f32,

    #[egui_probe(with custom_probe)]
    custom: Foo,

    #[egui_probe(name = "renamed ^_^")]
    renamed: u8,

    #[egui_probe(default = true)]
    maybe_boolean: Option<bool>,

    inner: InnerValue,

    inlined_tags: InlinedTags,

    option_combobox_tags: Option<ComboBoxTags>,

    array: [u8; 3],

    vector: Vec<bool>,

    #[egui_probe(frozen)]
    frozen_vector: Vec<bool>,

    map: HashMap<String, u32>,

    #[egui_probe(frozen)]
    frozen_map: HashMap<String, u32>,
}

struct EguiProbeDemoApp {
    value: DemoValue,
}

impl EguiProbeDemoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        EguiProbeDemoApp {
            value: DemoValue {
                boolean: false,
                boolean_toggle: false,
                float: 0.0,
                range: 22,
                maybe_range_to: Some(UpTo7(0)),
                range_from: 100,
                angle: 0.0,
                custom: Foo,
                renamed: 0,
                maybe_boolean: None,
                inner: InnerValue {
                    line: "Hello, world!".to_owned(),
                    multi_line: "Hello,\nworld!".to_owned(),
                    #[cfg(feature = "smallvec1")]
                    small_vec_1: smallvec1::smallvec!["First 4 values is on stack".to_owned()],
                    #[cfg(feature = "smallvec2")]
                    small_vec_2: smallvec2::smallvec![42.],
                    #[cfg(feature = "hashbrown")]
                    hash_brown: Default::default(),
                },
                inlined_tags: InlinedTags::Empty,
                option_combobox_tags: None,
                array: [0, 1, 2],
                vector: vec![false, true, false],
                frozen_vector: vec![false, true, false],

                map: {
                    let mut map = HashMap::new();
                    map.insert("foo".to_owned(), 1);
                    map.insert("bar".to_owned(), 2);
                    map
                },

                frozen_map: {
                    let mut map = HashMap::new();
                    map.insert("foo".to_owned(), 1);
                    map.insert("bar".to_owned(), 2);
                    map
                },
            },
        }
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
