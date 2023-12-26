use egui_probe::{angle, Probe};
use egui_probe_proc::EguiProbe;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-probe demo app",
        native_options,
        Box::new(|cc| Box::new(EguiProbeDemoApp::new(cc))),
    )
    .unwrap();
}

struct Foo;

fn custom_probe(_: &mut Foo, ui: &mut egui::Ui, _: &egui_probe::Style) -> egui::Response {
    ui.label("This is custom probe")
}

#[derive(EguiProbe)]
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

#[derive(EguiProbe)]
struct DemoValue {
    boolean: bool,

    #[egui_probe(toggle_switch)]
    boolean_toggle: bool,

    float: f32,

    #[egui_probe(range = 22..=55)]
    range: usize,

    range_to: UpTo7,

    #[egui_probe(range = 50..)]
    range_from: u8,

    #[egui_probe(as angle)]
    angle: f32,

    #[egui_probe(with custom_probe)]
    custom: Foo,

    #[egui_probe(name = "renamed ^_^")]
    renamed: u8,

    maybe_boolean: Option<bool>,

    inner: InnerValue,

    inlined_tags: InlinedTags,

    option_combobox_tags: Option<ComboBoxTags>,
}

#[derive(Default, EguiProbe)]
#[egui_probe(rename_all = camelCase)]
struct InnerValue {
    line: String,

    #[egui_probe(multiline)]
    multi_line: String,
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
                range_to: UpTo7(0),
                range_from: 100,
                angle: 0.0,
                custom: Foo,
                renamed: 0,
                inner: InnerValue {
                    line: "Hello, world!".to_owned(),
                    multi_line: "Hello,\nworld!".to_owned(),
                },
                maybe_boolean: None,
                inlined_tags: InlinedTags::Empty,
                option_combobox_tags: None,
            },
        }
    }
}

impl eframe::App for EguiProbeDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|style| style.animation_time = 5.0);

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            Probe::new("Value", &mut self.value).show(ui);
        });
    }
}
