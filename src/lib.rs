//! # egui-probe
//!
//! This libraty provides a trait for value editing in egui.
//! Powerful derive macros are provided for easy implementation on structs and enums.

mod array;
mod boolean;
mod collections;
mod map;
mod num;
mod option;
mod set;
mod text;
mod vec;
mod widget;

pub use egui;

pub use self::widget::{Probe, ProbeLayout};

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

/// Controls the style of probbing UI.
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
    fn add_button_text(&self) -> String {
        self.add_button_char.unwrap_or('+').to_string()
    }

    fn remove_button_text(&self) -> String {
        self.remove_button_char.unwrap_or('-').to_string()
    }
}

/// Provides ability to show probbing UI to values.
pub trait EguiProbe {
    /// Shows probbing UI to edit the value.
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response;

    /// Returns `true` if the value has sub-values for probbing.
    /// This will cause the value to be
    #[inline(always)]
    fn has_inner(&self) -> bool {
        false
    }

    /// Shows probbing UI to edit the inner values.
    ///
    /// It should add pairs of widgets to the UI for each record.
    /// If record has sub-records it should flatten them.
    #[inline(always)]
    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        let _ = f;
    }
}

impl<F> EguiProbe for F
where
    F: FnMut(&mut egui::Ui, &Style) -> egui::Response,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        (self)(ui, style)
    }
}

pub fn angle(value: &mut f32) -> impl EguiProbe + '_ {
    move |ui: &mut egui::Ui, _style: &Style| ui.drag_angle(value)
}

#[cfg(feature = "derive")]
pub use egui_probe_proc::EguiProbe;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub mod private {
    use crate::{
        boolean::toggle_switch, collections::EguiProbeFrozen, num::EguiProbeRange,
        text::EguiProbeMultiline,
    };

    use super::*;

    pub use core::stringify;

    #[inline(always)]
    pub fn probe_with<'a, T, F>(mut f: F, value: &'a mut T) -> impl EguiProbe + 'a
    where
        F: FnMut(&mut T, &mut egui::Ui, &Style) -> egui::Response + 'a,
    {
        move |ui: &mut egui::Ui, style: &Style| f(value, ui, style)
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
    pub fn probe_range<'a, T, R>(range: R, value: &'a mut T) -> EguiProbeRange<'a, T, R>
    where
        EguiProbeRange<'a, T, R>: EguiProbe,
    {
        EguiProbeRange { value, range }
    }

    #[inline(always)]
    pub fn probe_multiline<'a, T>(string: &'a mut T) -> EguiProbeMultiline<'a, T>
    where
        EguiProbeMultiline<'a, T>: EguiProbe,
    {
        EguiProbeMultiline { string }
    }

    #[inline(always)]
    pub fn probe_toggle_switch<'a>(value: &'a mut bool) -> impl EguiProbe + 'a {
        move |ui: &mut egui::Ui, _: &Style| toggle_switch(value, ui)
    }

    #[inline(always)]
    pub fn probe_frozen<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeFrozen<'a, T>: EguiProbe,
    {
        EguiProbeFrozen { value }
    }
}
