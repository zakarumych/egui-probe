use crate::EguiProbe;

impl<T, const N: usize> EguiProbe for [T; N]
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{N}]"))
    }

    fn has_inner(&self) -> bool {
        N != 0
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        for (i, value) in self.iter_mut().enumerate() {
            f(&format!("[{i}]"), value);
        }
    }
}

impl<T, const N: usize> EguiProbe for &mut [T; N]
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{N}]"))
    }

    fn has_inner(&self) -> bool {
        N != 0
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        for (i, value) in self.iter_mut().enumerate() {
            f(&format!("[{i}]"), value);
        }
    }
}

impl<T> EguiProbe for &mut [T]
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{}]", self.len()))
    }

    fn has_inner(&self) -> bool {
        !self.is_empty()
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        for (i, value) in self.iter_mut().enumerate() {
            f(&format!("[{i}]"), value);
        }
    }
}
