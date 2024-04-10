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
    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        f("color", &mut self.color);
        f("width", &mut non_negative(&mut self.width));
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
    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        f("top", &mut non_negative(&mut self.top));
        f("left", &mut non_negative(&mut self.left));
        f("bottom", &mut non_negative(&mut self.bottom));
        f("right", &mut non_negative(&mut self.right));
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
    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        f("nw", &mut non_negative(&mut self.nw));
        f("ne", &mut non_negative(&mut self.ne));
        f("sw", &mut non_negative(&mut self.sw));
        f("se", &mut non_negative(&mut self.se));
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

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        f("offset", &mut self.offset);
        f("blur", &mut non_negative(&mut self.blur));
        f("spread", &mut non_negative(&mut self.spread));
        f("color", &mut self.color);
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
    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        f("inner_margin", &mut self.inner_margin);
        f("outer_margin", &mut self.outer_margin);
        f("rounding", &mut self.rounding);
        f("shadow", &mut self.shadow);
        f("fill", &mut self.fill);
        f("stroke", &mut self.stroke);
    }
}
