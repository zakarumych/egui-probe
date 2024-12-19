use egui::{epaint::Hsva, Color32, Rgba};

use crate::EguiProbe;

/// Modifier to edit color as rgb.
pub struct EguiProbeRgb<'a, T> {
    pub value: &'a mut T,
}

/// Modifier to edit color as rgba.
pub struct EguiProbeRgba<'a, T> {
    pub value: &'a mut T,
}

/// Modifier to edit color as rgba.
pub struct EguiProbeRgbaPremultiplied<'a, T> {
    pub value: &'a mut T,
}

/// Modifier to edit color as rgba.
pub struct EguiProbeRgbaUnmultiplied<'a, T> {
    pub value: &'a mut T,
}

impl EguiProbe for Color32 {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.color_edit_button_srgba(self)
    }
}

impl EguiProbe for EguiProbeRgb<'_, Color32> {
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        egui::color_picker::color_edit_button_srgba(
            ui,
            self.value,
            egui::color_picker::Alpha::Opaque,
        )
    }
}

impl EguiProbe for EguiProbeRgba<'_, Color32> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.color_edit_button_srgba(self.value)
    }
}

impl EguiProbe for EguiProbeRgbaPremultiplied<'_, Color32> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.color_edit_button_srgba(self.value)
    }
}

impl EguiProbe for Rgba {
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        egui::color_picker::color_edit_button_rgba(
            ui,
            self,
            egui::color_picker::Alpha::BlendOrAdditive,
        )
    }
}

impl EguiProbe for EguiProbeRgb<'_, Rgba> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        egui::color_picker::color_edit_button_rgba(
            ui,
            self.value,
            egui::color_picker::Alpha::Opaque,
        )
    }
}

impl EguiProbe for EguiProbeRgba<'_, Rgba> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        egui::color_picker::color_edit_button_rgba(
            ui,
            self.value,
            egui::color_picker::Alpha::Opaque,
        )
    }
}

impl EguiProbe for EguiProbeRgbaUnmultiplied<'_, Rgba> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        egui::color_picker::color_edit_button_rgba(
            ui,
            self.value,
            egui::color_picker::Alpha::Opaque,
        )
    }
}

impl EguiProbe for Hsva {
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        egui::color_picker::color_edit_button_hsva(
            ui,
            self,
            egui::color_picker::Alpha::BlendOrAdditive,
        )
    }
}
