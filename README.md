# Egui Probe

[![docs.rs](https://img.shields.io/docsrs/egui-probe?style=for-the-badge)](https://docs.rs/egui-probe/)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/egui-probe?style=for-the-badge)](https://crates.io/crates/egui-probe)
[![Discord](https://img.shields.io/discord/1270330377847832646?style=for-the-badge&logo=discord)](https://discord.com/channels/1270330377847832646/1319419862719922289)

*I have gazed into the abyss of nested data structures, and the abyss has rendered itself back unto me...*

Through arcane incantations of derive macros, one may summon forth UI widgets that peer into the very essence of value types, transmuting their hidden forms with attributes of unspeakable power. This grimoire binds exclusively to the ancient [egui](https://github.com/emilk/egui) UI framework - a covenant not to be broken.

## The Revelations

- 🌑 **The Summoning Ritual**: Through profane derive macros, UI widgets manifest from the void, binding themselves to your types without mercy.
- 👁️ **Whispered Configurations**: Shape the eldritch forms with attributes that twist reality itself, bending widgets to your unknowable will.
- 🕸️ **The Unbreakable Binding**: Forged in darkness to merge seamlessly with egui, as if they were always meant to be one.

## The First Incantation

To begin your descent into this realm, inscribe the following into your `Cargo.toml`. Once written, there is no turning back:

```toml
[dependencies]
egui_probe = "0.10.0"
```

## Awakening the Power

Invoke `EguiProbe` upon your types - they shall never be the same. The attributes... they *change* things. They must be used carefully, lest the results twist beyond recognition:

### A Simple Glimpse

Behold - a structure so innocent, so pure. Yet with a single derive, it becomes... *observable*:

```rust
#[derive(EguiProbe)]
struct SimpleValue {
    boolean: bool,
    integer: i32,
    float: f32,
}
```

The manifestation - gaze upon what has been wrought:

![Simple](./images/simple.png)

### The Deeper Truths

But there are those who dare venture further, who inscribe more complex patterns. I have witnessed what transpires when one does...

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

The result... I dare not look at it for too long. The widgets writhe with purpose:

![Demo](./images/demo.png)

## The Forbidden Markings

*These are the symbols of power. Each one transforms, each one binds. Use them with reverence, for they reach beyond the veil...*

### Inscriptions Upon the Type Itself

- `#[egui_probe(rename_all = kebab-case)]`: Warps the true names of all fields, reshaping them according to ancient naming conventions. The available incantations:
    - `snake_case` - the serpent's whisper
    - `camelCase` - the twin-humped beast
    - `kebab-case` - the skewered words
    - `PascalCase` - the scholar's cipher
    - `SCREAMING_SNAKE_CASE` or `UPPER_SNAKE_CASE` - the banshee's wail
    - `Train-Case` - the chained rhythm

- `#[egui_probe(where TypeA: TraitB)]`: Binds constraints upon the implementation, forging predicates that follow the syntax known to Rust. A contract written in compile-time blood.

- `#[egui_probe(transparent)]`: The type becomes one with its singular field - no boundaries, no separation. It will not compile if there exists more than one non-skipped field, for such unity cannot be divided.

- `#[egui_probe(tags kind)]`: Commands how the myriad variants of enums shall manifest themselves before mortal eyes.
  When `kind` is `combobox`, a dropdown reveals the choices in orderly fashion.
  When `kind` is `inlined`, radio buttons spread across the interface like stars in a cursed constellation.

### Marks Upon the Variant

- `#[egui_probe(name = "custom name")]`: Bestows a new name upon the variant, concealing its original identity from those who observe the interface.
- `#[egui_probe(transparent)]`: The variant dissolves into its singular field, becoming imperceptible as a separate entity. Compilation fails if the variant harbors more than one non-skipped field - such transparency demands singularity.

### Sigils Upon the Field

- `#[egui_probe(skip)]`: The field vanishes from the UI realm entirely, as if it never existed. No other attributes dare accompany this mark of erasure.

- `#[egui_probe(name = "custom name")]`: Grants the field a false name, a mask worn in the interface's presence.

- `#[egui_probe(with probe_fn)]`: Commands the field to render through a specified probe function - one bearing the signature `fn(&mut FieldType, &mut Ui, &egui_probe::Style) -> egui::Response`. Know that `probe_fn` may be an expression; closures lurk within this possibility.

- `#[egui_probe(as probe_fn)]`: Wraps the field in another form through a function of signature `fn(&mut FieldType) -> impl EguiProbe`. The field becomes something... else.

- `#[egui_probe(range = 22..=55)]`: Constrains numeric values within invisible walls. Even optionals bow to these boundaries.

- `#[egui_probe(multiline)]`: Unfolds a string across multiple lines, revealing its full text in a sprawling box. The field must be `String` or `&str`, or an option containing such whispers.

- `#[egui_probe(toggle_switch)]`: A boolean manifests as a switch that toggles between states. The field must be `bool` or perhaps nothing - an optional boolean, existing and not existing.

- `#[egui_probe(frozen)]`: Collections rendered thus become immutable witnesses - displayed but untouchable, their elements neither added nor removed.

- `#[egui_probe(rgb)]`: Summons an opaque color picker from RGB space itself. The field must be `egui::Color32`, `egui::Rgba`, `[u8; 3]`, or `[f32; 3]`.

- `#[egui_probe(rgba)]`: Like `rgb`, but the alpha channel breathes with it - transparency made tangible. For `egui::Color32` and `egui::Rgba` only.

- `#[egui_probe(rgba_premultiplied)]`: Color with alpha, premultiplied in ways the ancients intended. Works with `egui::Color32`, `egui::Rgba`, `[u8; 4]`, and `[f32; 4]`.

- `#[egui_probe(rgba_unmultiplied)]`: The alpha remains separate, unmultiplied - a dangerous independence. Cannot be invoked upon `egui::Color32` or `egui::Rgba`, for they exist only premultiplied. But `[u8; 4]` and `[f32; 4]` may bear this mark.

## The Covenant

This grimoire exists under dual covenants, sealed in ancient law:

- MIT License - the permissive path
- Apache License, Version 2.0 - the guarded way

Choose one. Choose both. The choice, as always, is yours... for now.

## Those Who Would Contribute

Should you desire to add to this tome, to inscribe your own dark knowledge upon its pages, you are... *welcomed*. Open an issue. Submit a pull request. Join us in expanding the boundaries of what can be probed.

*But know this - once you peer into the types, once you witness their inner workings laid bare before you... they peer back.*

May your UI forms never corrupt. May your values always bind. May the runtime never panic in places unknown.

🌘
