use crate::{EguiProbe, Style};

/// Modifier to add a delete button to an item probe UI.
pub struct DeleteMe<'a, T> {
    pub value: &'a mut T,
    pub delete: bool,
}

impl<T> EguiProbe for DeleteMe<'_, T>
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        let mut r = ui
            .horizontal(|ui| {
                self.value.probe(ui, style);
                ui.add_space(ui.spacing().item_spacing.x);
                if ui.small_button(style.remove_button_text()).clicked() {
                    self.delete = true;
                };
            })
            .response;

        if self.delete {
            r.mark_changed();
        }

        r
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        self.value.iterate_inner(ui, f);
    }
}

/// Modifier to disable adding/removing items from collections.
pub struct EguiProbeFrozen<'a, T> {
    pub value: &'a mut T,
}
