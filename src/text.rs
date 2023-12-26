use crate::{option::option_probe_with, EguiProbe, Style};

impl EguiProbe for String {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::singleline(self))
    }
}

impl EguiProbe for &str {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::singleline(self))
    }
}

/// Wrapper for string-like types to show multiline text field.
pub struct EguiProbeMultiline<'a, T> {
    pub string: &'a mut T,
}

impl EguiProbe for EguiProbeMultiline<'_, String> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::multiline(self.string))
    }
}

impl EguiProbe for EguiProbeMultiline<'_, &str> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::multiline(self.string))
    }
}

impl EguiProbe for EguiProbeMultiline<'_, Option<String>> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        option_probe_with(self.string, ui, style, |string, ui, _| {
            ui.add(egui::TextEdit::multiline(string));
        })
    }
}

impl EguiProbe for EguiProbeMultiline<'_, Option<&str>> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        option_probe_with(self.string, ui, &Style::default(), |string, ui, _| {
            ui.add(egui::TextEdit::multiline(string));
        })
    }
}
