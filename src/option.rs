use crate::{EguiProbe, Style};

impl<T> EguiProbe for Option<T>
where
    T: EguiProbe + Default,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        option_probe_with(self, ui, style, |value, ui, style| {
            value.probe(ui, style);
        })
    }

    #[inline(always)]
    fn has_inner(&mut self) -> bool {
        match self {
            Some(value) => value.has_inner(),
            None => false,
        }
    }

    #[inline(always)]
    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        if let Some(value) = self {
            value.iterate_inner(f);
        }
    }
}

#[inline(always)]
pub fn option_probe_with<T>(
    value: &mut Option<T>,
    ui: &mut egui::Ui,
    style: &Style,
    probe: impl FnOnce(&mut T, &mut egui::Ui, &Style),
) -> egui::Response
where
    T: Default,
{
    ui.horizontal(|ui| {
        let mut checked = value.is_some();

        if ui.selectable_label(!checked, "None").clicked() {
            checked = false;
        }
        if ui.selectable_label(checked, "Some").clicked() {
            checked = true;
        }
        if checked != value.is_some() {
            *value = checked.then(T::default);
        }
        if let Some(value) = value {
            probe(value, ui, style);
        }
    })
    .response
}
