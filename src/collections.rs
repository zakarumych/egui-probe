use crate::EguiProbe;

/// Modifier to add a delete button to an item probe UI.
pub struct DeleteMe<'a, T> {
    pub value: &'a mut T,
    pub delete: bool,
}

impl<T> EguiProbe for DeleteMe<'_, T>
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        ui.horizontal(|ui| {
            self.value.probe(ui, style);
            ui.add_space(ui.spacing().item_spacing.x);
            if ui.small_button(style.remove_button_text()).clicked() {
                self.delete = true;
            };
        })
        .response
    }

    fn has_inner(&self) -> bool {
        self.value.has_inner() && !self.delete
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        self.value.iterate_inner(f);
    }
}

/// Modifier to disable adding/removing items from collections.
pub struct EguiProbeFrozen<'a, T> {
    pub value: &'a mut T,
}
