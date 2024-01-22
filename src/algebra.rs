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
            ui.label("top");
            self.min.y.probe(ui, style);
            ui.label("left");
            self.min.x.probe(ui, style);
            ui.label("bottom");
            self.max.y.probe(ui, style);
            ui.label("right");
            self.max.x.probe(ui, style);
        })
        .response
    }
}
