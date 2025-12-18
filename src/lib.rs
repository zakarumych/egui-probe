//! # Egui Probe
//!
//! *I have gazed into the abyss of nested data structures, and the abyss has rendered itself back unto me...*
//!
//! Through arcane incantations of derive macros, one may summon forth UI widgets that peer into the very essence
//! of value types, transmuting their hidden forms with attributes of unspeakable power. This grimoire binds exclusively
//! to the ancient [egui](https://github.com/emilk/egui) UI framework—a covenant not to be broken.
//!
//! ## The Revelations
//!
//! - 🌑 **The Summoning Ritual**: Through profane derive macros, UI widgets manifest from the void, binding themselves to your types without mercy.
//! - 👁️ **Whispered Configurations**: Shape the eldritch forms with attributes that twist reality itself, bending widgets to your unknowable will.
//! - 🕸️ **The Unbreakable Binding**: Forged in darkness to merge seamlessly with egui, as if they were always meant to be one.
//!
//! ## The First Incantation
//!
//! To begin your descent into this realm, inscribe the following into your `Cargo.toml`. Once written, there is no turning back:
//!
//! ```toml
//! [dependencies]
//! egui_probe = "0.5.2"
//! ```
//!
//! ## Awakening the Power
//!
//! Invoke `EguiProbe` upon your types—they shall never be the same. The attributes... they *change* things:
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(
    not(feature = "derive"),
    doc = "```ignore\n// This example requires the `derive` feature."
)]
//! use egui_probe::{EguiProbe, Probe, angle};
//! use eframe::App;
//!
//! #[derive(EguiProbe)]
//! struct DemoValue {
//!     boolean: bool,
//!
//!     #[egui_probe(toggle_switch)]
//!     boolean_toggle: bool,
//!
//!     float: f32,
//!
//!     #[egui_probe(range = 22..=55)]
//!     range: usize,
//!
//!     #[egui_probe(as angle)]
//!     angle: f32,
//!
//!     #[egui_probe(name = "renamed ^_^")]
//!     renamed: u8,
//!
//!     inner: InnerValue,
//! }
//!
//! #[derive(Default, EguiProbe)]
//! struct InnerValue {
//!     line: String,
//!
//!     #[egui_probe(multiline)]
//!     multi_line: String,
//! }
//!
//! struct EguiProbeDemoApp {
//!     value: DemoValue,
//! }
//!
//! impl App for EguiProbeDemoApp {
//!     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//!         egui::CentralPanel::default().show(ctx, |ui| {
//!             Probe::new(&mut self.value).show(ui);
//!         });
//!     }
//! }
//! ```
//!
//! ## The Forbidden Markings
//!
//! *These are but a few of the symbols. Each transforms, each binds:*
//!
//! - `#[egui_probe(toggle_switch)]`: A boolean manifests as a switch between two states.
//! - `#[egui_probe(range = 22..=55)]`: Numeric values constrained within invisible walls.
//! - `#[egui_probe(as angle)]`: A float rendered as an angle—rotation made manifest.
//! - `#[egui_probe(name = "custom name")]`: A false name, a mask worn before observers.
//! - `#[egui_probe(multiline)]`: Strings unfold across the void, revealing their length.
//!
//! ## The Covenant
//!
//! This grimoire exists under dual covenants:
//!
//! - MIT License
//! - Apache License, Version 2.0
//!
//! Choose one. Choose both. The choice is yours.
//!
//! ## Those Who Would Contribute
//!
//! Should you desire to add to this tome, open an issue or submit a pull request.
//!
//! *Once you peer into the types, they peer back.*
//!
//! 🌘
#![allow(clippy::inline_always, clippy::use_self)]

mod algebra;
mod array;
mod boolean;
mod collections;
mod color;
#[cfg(feature = "hashbrown")]
mod hashbrown;
mod map;
mod num;
mod option;
mod set;
#[cfg(any(feature = "smallvec1", feature = "smallvec2"))]
mod small_vec;
mod text;
mod ui;
mod vec;
mod widget;

pub use egui;

pub use self::{
    boolean::toggle_switch,
    collections::DeleteMe,
    option::option_probe_with,
    widget::{Probe, ProbeLayout},
};

/// The manner in which boolean values reveal themselves—checkbox or switch,
/// each a gateway to binary truth.
#[derive(Clone, Copy, Debug)]
pub enum BooleanStyle {
    Checkbox,
    ToggleSwitch,
}

impl Default for BooleanStyle {
    #[inline]
    fn default() -> Self {
        Self::Checkbox
    }
}

/// How enum variants manifest before mortal eyes—inlined across space,
/// or collapsed within a combobox dropdown. Choose wisely.
#[derive(Clone, Copy, Debug)]
pub enum VariantsStyle {
    Inlined,
    ComboBox,
}

impl Default for VariantsStyle {
    #[inline]
    fn default() -> Self {
        Self::ComboBox
    }
}

/// Commands the aesthetic form of the probing interface—the visual rules
/// by which the unseen becomes seen. Tread carefully when altering these configurations.
#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub boolean: BooleanStyle,
    pub variants: VariantsStyle,
    pub field_indent_size: Option<f32>,
    pub add_button_char: Option<char>,
    pub remove_button_char: Option<char>,
}

impl Default for Style {
    #[inline]
    fn default() -> Self {
        Style {
            boolean: BooleanStyle::default(),
            variants: VariantsStyle::default(),
            field_indent_size: None,
            add_button_char: None,
            remove_button_char: None,
        }
    }
}

impl Style {
    #[must_use]
    pub fn add_button_text(&self) -> String {
        self.add_button_char.unwrap_or('+').to_string()
    }

    #[must_use]
    pub fn remove_button_text(&self) -> String {
        self.remove_button_char.unwrap_or('-').to_string()
    }
}

/// Grants the ability to peer into values, to observe and manipulate them through the UI.
/// Those who implement this trait surrender their privacy to the interface.
pub trait EguiProbe {
    /// Manifests the probing UI—a window through which the value may be observed and altered.
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response;

    /// Traverses the inner depths, revealing sub-values through iteration.
    /// Each record exposed, each secret laid bare. If sub-records exist, they too shall be flattened.
    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        let _ = (ui, f);
    }
}

impl<P> EguiProbe for &mut P
where
    P: EguiProbe,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        P::probe(*self, ui, style)
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        P::iterate_inner(*self, ui, f);
    }
}

impl<P> EguiProbe for Box<P>
where
    P: EguiProbe,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        P::probe(&mut *self, ui, style)
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        P::iterate_inner(&mut *self, ui, f);
    }
}

/// A function transformed into something probe-able—wrapped in a form
/// that the interface can comprehend and invoke.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EguiProbeFn<F>(pub F);

impl<F> EguiProbe for EguiProbeFn<F>
where
    F: FnMut(&mut egui::Ui, &Style) -> egui::Response,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        (self.0)(ui, style)
    }
}

/// Transmutes a function into something the probe can witness and wield.
#[inline(always)]
pub const fn probe_fn<F>(f: F) -> EguiProbeFn<F> {
    EguiProbeFn(f)
}

/// Renders a float as an angle—revealing rotation, arc, the turning of unseen gears.
#[inline(always)]
pub fn angle(value: &mut f32) -> impl EguiProbe + '_ {
    probe_fn(move |ui: &mut egui::Ui, _style: &Style| ui.drag_angle(value))
}

pub mod customize {
    use std::ops::RangeFull;

    use super::{
        EguiProbe, Style,
        boolean::ToggleSwitch,
        collections::EguiProbeFrozen,
        color::{
            EguiProbeRgb, EguiProbeRgba, EguiProbeRgbaPremultiplied, EguiProbeRgbaUnmultiplied,
        },
        egui,
        num::{EguiProbeRange, StepUnset},
        probe_fn,
        text::EguiProbeMultiline,
    };

    #[inline(always)]
    pub fn probe_with<'a, T, F>(mut f: F, value: &'a mut T) -> impl EguiProbe + 'a
    where
        F: FnMut(&mut T, &mut egui::Ui, &Style) -> egui::Response + 'a,
    {
        probe_fn(move |ui: &mut egui::Ui, style: &Style| f(value, ui, style))
    }

    #[inline(always)]
    pub fn probe_as<'a, T, F, R>(f: F, value: &'a mut T) -> impl EguiProbe + 'a
    where
        F: FnOnce(&'a mut T) -> R,
        R: EguiProbe + 'a,
    {
        f(value)
    }

    #[inline(always)]
    pub const fn probe_range<'a, T, R>(range: R, value: &'a mut T) -> EguiProbeRange<'a, T, R>
    where
        EguiProbeRange<'a, T, R>: EguiProbe,
    {
        EguiProbeRange {
            value,
            range,
            step: StepUnset,
        }
    }

    #[inline(always)]
    pub const fn probe_range_step<'a, T, R, S>(
        range: R,
        step: S,
        value: &'a mut T,
    ) -> EguiProbeRange<'a, T, R, S>
    where
        EguiProbeRange<'a, T, R, S>: EguiProbe,
    {
        EguiProbeRange { value, range, step }
    }

    #[inline(always)]
    pub const fn probe_step<'a, T, S>(
        step: S,
        value: &'a mut T,
    ) -> EguiProbeRange<'a, T, RangeFull, S>
    where
        EguiProbeRange<'a, T, RangeFull, S>: EguiProbe,
    {
        EguiProbeRange {
            value,
            range: ..,
            step,
        }
    }

    #[inline(always)]
    pub const fn probe_multiline<'a, T>(string: &'a mut T) -> EguiProbeMultiline<'a, T>
    where
        EguiProbeMultiline<'a, T>: EguiProbe,
    {
        EguiProbeMultiline { string }
    }

    #[inline(always)]
    pub fn probe_toggle_switch<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        ToggleSwitch<'a, T>: EguiProbe,
    {
        ToggleSwitch(value)
    }

    #[inline(always)]
    pub fn probe_frozen<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeFrozen<'a, T>: EguiProbe,
    {
        EguiProbeFrozen { value }
    }

    #[inline(always)]
    pub fn probe_rgb<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgb<'a, T>: EguiProbe,
    {
        EguiProbeRgb { value }
    }

    #[inline(always)]
    pub fn probe_rgba<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgba<'a, T>: EguiProbe,
    {
        EguiProbeRgba { value }
    }

    #[inline(always)]
    pub fn probe_rgba_premultiplied<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgbaPremultiplied<'a, T>: EguiProbe,
    {
        EguiProbeRgbaPremultiplied { value }
    }

    #[inline(always)]
    pub fn probe_rgba_unmultiplied<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgbaUnmultiplied<'a, T>: EguiProbe,
    {
        EguiProbeRgbaUnmultiplied { value }
    }
}

#[cfg(feature = "derive")]
pub use egui_probe_proc::EguiProbe;

#[cfg(feature = "derive")]
extern crate self as egui_probe;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub mod private {
    pub use super::customize::*;
    pub use core::stringify;
}

#[cfg(feature = "derive")]
#[test]
fn test_all_attributes() {
    #![allow(unused)]

    trait A {}

    #[derive(EguiProbe)]
    #[egui_probe(where T: EguiProbe)]
    struct TypeAttributes<T> {
        a: T,
    }

    struct NoProbe;

    #[derive(EguiProbe)]
    #[egui_probe(rename_all = Train-Case)]
    struct FieldAttributes {
        #[egui_probe(skip)]
        skipped: NoProbe,

        #[egui_probe(name = "renamed")]
        a: u8,

        #[egui_probe(with |_, ui, _| ui.label("a label"))]
        b: u8,

        #[egui_probe(as angle)]
        c: f32,

        #[egui_probe(range = 0..=100)]
        d: u8,

        #[egui_probe(multiline)]
        e: String,

        #[egui_probe(multiline)]
        f: Option<String>,

        #[egui_probe(toggle_switch)]
        g: bool,

        #[egui_probe(toggle_switch)]
        h: Option<bool>,

        #[egui_probe(frozen)]
        i: Vec<u8>,

        #[egui_probe(rgb)]
        j: egui::Color32,

        #[egui_probe(rgba)]
        k: egui::Color32,

        #[egui_probe(rgba_premultiplied)]
        l: [u8; 4],

        #[egui_probe(rgba_unmultiplied)]
        m: [f32; 4],
    }

    #[derive(EguiProbe)]
    #[egui_probe(tags combobox)]
    enum EnumAttributes {
        #[egui_probe(name = "renamed")]
        A,

        #[egui_probe(transparent)]
        B {
            #[egui_probe(skip)]
            skipped: (),

            b: f32,
        },
    }
}
