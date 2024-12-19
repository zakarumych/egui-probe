# Egui Probe

Effortlessly create UI widgets to display and modify value types using a derive macro with rich customization via attributes. This library is exclusively for the [egui](https://github.com/emilk/egui) UI framework.

## Features

- 🪄 **Derive Macro**: Automatically generate UI widgets for your types.
- 🎨 **Rich Customization**: Customize the generated widgets using attributes.
- 🚀 **Seamless Integration**: Designed to work seamlessly with egui.

## Getting Started

Add `egui_probe` to your `Cargo.toml`:

```toml
[dependencies]
egui_probe = "0.5.2"
```

## Usage

Derive `EguiProbe` for your types and use attributes to customize the UI:

```rust
use egui_probe::{EguiProbe, Probe, angle};
use eframe::App;

#[derive(EguiProbe)]
struct DemoValue {
    boolean: bool,

    #[egui_probe(toggle_switch)]
    boolean_toggle: bool,

    float: f32,

    #[egui_probe(range = 22..=55)]
    range: usize,

    #[egui_probe(as angle)]
    angle: f32,

    #[egui_probe(name = "renamed ^_^")]
    renamed: u8,

    inner: InnerValue,
}

#[derive(Default, EguiProbe)]
struct InnerValue {
    line: String,

    #[egui_probe(multiline)]
    multi_line: String,
}

struct EguiProbeDemoApp {
    value: DemoValue,
}

impl App for EguiProbeDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Probe::new(&mut self.value).show(ui);
        });
    }
}
```

## Attributes

- `#[egui_probe(toggle_switch)]`: Render a boolean as a toggle switch.
- `#[egui_probe(range = 22..=55)]`: Specify a range for numeric values.
- `#[egui_probe(as angle)]`: Render a float as an angle.
- `#[egui_probe(name = "custom name")]`: Rename the field in the UI.
- `#[egui_probe(multiline)]`: Render a string as a multiline text box.

## License

This project is licensed under either of

- MIT License
- Apache License, Version 2.0

at your option.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

Enjoy building your UI with Egui Probe! 🚀
