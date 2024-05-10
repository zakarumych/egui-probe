use egui::{epaint::Shadow, Frame, Margin, Rounding, Stroke};

use crate::{num::non_negative, EguiProbe};

impl EguiProbe for Stroke {
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak("Stroke")
    }

    #[inline(always)]
    fn has_inner(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn iterate_inner(&mut self, ui: &mut egui::Ui, f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe)) {
        f("color", ui, &mut self.color);
        f("width", ui, &mut non_negative(&mut self.width));
    }
}

impl EguiProbe for Margin {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak("Margin")
    }

    #[inline(always)]
    fn has_inner(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn iterate_inner(&mut self, ui: &mut egui::Ui, f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe)) {
        f("top", ui, &mut non_negative(&mut self.top));
        f("left", ui, &mut non_negative(&mut self.left));
        f("bottom", ui, &mut non_negative(&mut self.bottom));
        f("right", ui, &mut non_negative(&mut self.right));
    }
}

impl EguiProbe for Rounding {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak("Rounding")
    }

    #[inline(always)]
    fn has_inner(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn iterate_inner(&mut self, ui: &mut egui::Ui, f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe)) {
        f("nw", ui, &mut non_negative(&mut self.nw));
        f("ne", ui, &mut non_negative(&mut self.ne));
        f("sw", ui, &mut non_negative(&mut self.sw));
        f("se", ui, &mut non_negative(&mut self.se));
    }
}

impl EguiProbe for Shadow {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak("Shadow")
    }

    #[inline(always)]
    fn has_inner(&mut self) -> bool {
        true
    }

    fn iterate_inner(&mut self, ui: &mut egui::Ui, f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe)) {
        f("offset", ui, &mut self.offset);
        f("blur", ui, &mut non_negative(&mut self.blur));
        f("spread", ui, &mut non_negative(&mut self.spread));
        f("color", ui, &mut self.color);
    }
}

impl EguiProbe for Frame {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak("Frame")
    }

    #[inline(always)]
    fn has_inner(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn iterate_inner(&mut self, ui: &mut egui::Ui, f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe)) {
        f("inner_margin", ui, &mut self.inner_margin);
        f("outer_margin", ui, &mut self.outer_margin);
        f("rounding", ui, &mut self.rounding);
        f("shadow", ui, &mut self.shadow);
        f("fill", ui, &mut self.fill);
        f("stroke", ui, &mut self.stroke);
    }
}
