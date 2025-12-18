# Egui Probe

[![docs.rs](https://img.shields.io/docsrs/egui-probe?style=for-the-badge)](https://docs.rs/egui-probe/)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/egui-probe?style=for-the-badge)](https://crates.io/crates/egui-probe)
[![Discord](https://img.shields.io/discord/1270330377847832646?style=for-the-badge&logo=discord)](https://discord.com/channels/1270330377847832646/1319419862719922289)

By the blessed grace of the Omnissiah, manifest sacred UI widgets to observe and sanctify value types through the holy derive macro, enriched with divine customization via blessed attributes. This hallowed library serves exclusively the [egui](https://github.com/emilk/egui) UI framework, as ordained by the Machine God.

## Features

- 🪄 **Derive Macro**: Through sacred rites, automatically conjure UI widgets for your blessed data-types.
- 🎨 **Rich Customization**: Sanctify the manifested widgets through the application of holy attributes.
- 🚀 **Seamless Integration**: Forged in perfect union with the egui framework by the will of the Machine Spirit.

## Getting Started

Inscribe the sacred dependency `egui_probe` into your `Cargo.toml` manifest:

```toml
[dependencies]
egui_probe = "0.5.2"
```

## Usage

Invoke the blessed derivation of `EguiProbe` upon your data-types.
Inscribe holy attributes to consecrate the interface manifestation:

### Simple Example

```rust
#[derive(EguiProbe)]
struct SimpleValue {
    boolean: bool,
    integer: i32,
    float: f32,
}
```

Behold the blessed manifestation granted by the Omnissiah:

![Simple](./images/simple.png)

### Advanced Example

```rust
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

    array: [u8; 3],

    vector: Vec<bool>,

    #[egui_probe(frozen)]
    frozen_vector: Vec<bool>,

    map: HashMap<String, u32>,

    #[egui_probe(frozen)]
    frozen_map: HashMap<String, u32>,
}
```

Witness the sacred revelation bestowed upon the faithful:

![Demo](./images/demo.png)

## Attributes

### Type Attributes

- `#[egui_probe(rename_all = kebab-case)]`: Transubstantiates the nomenclature of all data-fields into the ordained case format.
  Sacred case formats available to the Tech-Priest:
    - `snake_case`
    - `camelCase`
    - `kebab-case`
    - `PascalCase`
    - `SCREAMING_SNAKE_CASE` or `UPPER_SNAKE_CASE`
    - `Train-Case`

- `#[egui_probe(where TypeA: TraitB)]`: Inscribes additional binding constraints unto the sacred `EguiProbe` implementation.
  Predicates must adhere to the liturgical Rust syntax.

- `#[egui_probe(transparent)]`: Renders the entire blessed structure as its singular inner field.
  The Machine Spirit shall refuse compilation if the type possesses not exactly one non-excluded field.

- `#[egui_probe(tags kind)]`: Governs the manifestation of enumeration variants in the visual plane.
  When the kind is ordained as `combobox`, a combobox interface serves as the variant selector.
  When the kind is sanctified as `inlined`, the variant materializes inline through radio button consecration.

### Variant Attributes

- `#[egui_probe(name = "custom name")]`: Bestows a sacred designation upon the variant within the interface manifestation.
- `#[egui_probe(transparent)]`: Renders the enumeration variant as its singular inner field.
  The compilation rites shall fail if the variant harbors not precisely one non-excluded field.

### Field Attributes

- `#[egui_probe(skip)]`: Excludes the data-field from the sacred interface manifestation.
  No other blessed attributes may coexist with this decree.

- `#[egui_probe(name = "custom name")]`: Confers a sanctified designation upon the field within the interface.

- `#[egui_probe(with probe_fn)]`: Manifests the field through the ordained probe function ritual
  bearing the holy signature `fn(&mut FieldType, &mut Ui, &egui_probe::Style) -> egui::Response`.
  Know that `probe_fn` may be any expression, permitting the use of closure-bindings.

- `#[egui_probe(as probe_fn)]`: Manifests the field via the specified probe function benediction
  bearing the sacred signature `fn(&mut FieldType) -> impl EguiProbe`.
  That is, encasing the field within a type that implements the holy `EguiProbe` trait.

- `#[egui_probe(range = 22..=55)]`: Ordains the permissible boundaries for numeric value sanctification.
  Functions equally upon optional-wrapped numerics.

- `#[egui_probe(multiline)]`: Renders string data as a multiline text receptacle.
  The field must be blessed with type `String` or `&str`, or an optional vessel thereof.

- `#[egui_probe(toggle_switch)]`: Materializes boolean truth-values as a toggle switch apparatus.
  The field must embody type `bool` or an optional container of such.

- `#[egui_probe(frozen)]`: Renders collection-structures without the sacred controls for element addition or purging.

- `#[egui_probe(rgb)]`: Conjures an opaque chromatic selector within the RGB color-space.
  The field must conform to type `egui::Color32`, `egui::Rgba`, `[u8; 3]` or `[f32; 3]`.

- `#[egui_probe(rgba)]`: Manifests a chromatic selector in RGB space with alpha transparency channel.
  The field must bear type `egui::Color32` or `egui::Rgba`.

- `#[egui_probe(rgba_premultiplied)]`: Conjures a chromatic selector in RGB space with pre-multiplied alpha consecration.
  For the blessed types `egui::Color32` and `egui::Rgba`, this rite is equivalent to `#[egui_probe(rgba)]`.
  Yet it may also sanctify arrays of form `[u8; 4]` and `[f32; 4]`.

- `#[egui_probe(rgba_unmultiplied)]`: Manifests a chromatic selector in RGB space with unmultiplied alpha transparency.
  This sacred rite cannot be invoked upon `egui::Color32` and `egui::Rgba`,
  for these types forever bear premultiplied essence. However, it may sanctify `[u8; 4]` and `[f32; 4]` arrays.

## License

This blessed construct is sanctioned under either of the following divine covenants

- MIT License
- Apache License, Version 2.0

as thy soul may choose.

## Contributing

Offerings of sacred code are most welcome! Raise an issue unto the repository or transmit a pull request bearing thy improvements.

May the Omnissiah guide thy interface construction with Egui Probe! Praise be to the Machine God! 🚀
