use egui::{Pos2, Rect, Vec2};

use crate::EguiProbe;

impl EguiProbe for Vec2 {
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.x));
            ui.add(egui::DragValue::new(&mut self.y));
        })
        .response
    }
}

impl EguiProbe for Pos2 {
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.x));
            ui.add(egui::DragValue::new(&mut self.y));
        })
        .response
    }
}

impl EguiProbe for Rect {
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        ui.horizontal(|ui| {
            let top = ui.label("top");
            self.min.y.probe(ui, style).labelled_by(top.id);
            let left = ui.label("left");
            self.min.x.probe(ui, style).labelled_by(left.id);
            let bottom = ui.label("bottom");
            self.max.y.probe(ui, style).labelled_by(bottom.id);
            let right = ui.label("right");
            self.max.x.probe(ui, style).labelled_by(right.id);
        })
        .response
    }
}
