use core::ops::{RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

use crate::{option::option_probe_with, EguiProbe, Style};

/// Bundles value and a range to show probbing UI to edit the value in that range.
pub struct EguiProbeRange<'a, T, R> {
    pub value: &'a mut T,
    pub range: R,
}

macro_rules! impl_for_num_types {
    ($num_type:ident) => {
        impl EguiProbe for $num_type {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                ui.add(egui::DragValue::new(self))
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeFull> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = $num_type::MIN..=$num_type::MAX;
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(self.value).clamp_range(range));
                    ui.weak(format!("{}..={}", $num_type::MIN, $num_type::MAX));
                }).response
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeFrom<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(self.value).clamp_range(range));
                    ui.weak(format!("{}..={}", self.range.start, $num_type::MAX));
                }).response
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeToInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(self.value).clamp_range(range));
                    ui.weak(format!("{}..={}", $num_type::MIN, self.range.end));
                }).response
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.clone();
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(self.value).clamp_range(range));
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                }).response
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFull> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=$num_type::MAX;
                option_probe_with(self.value, ui, style, |value, ui, _| {
                    ui.add(egui::DragValue::new(value).clamp_range(range));
                    ui.weak(format!("{}..={}", $num_type::MIN, $num_type::MAX));
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFrom<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                option_probe_with(self.value, ui, style, |value, ui, _| {
                    ui.add(egui::DragValue::new(value).clamp_range(range));
                    ui.weak(format!("{}..={}", self.range.start, $num_type::MAX));
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeToInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                option_probe_with(self.value, ui, style, |value, ui, _| {
                    ui.add(egui::DragValue::new(value).clamp_range(range));
                    ui.weak(format!("{}..={}", $num_type::MIN, self.range.end));
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.clone();
                option_probe_with(self.value, ui, style, |value, ui, _| {
                    ui.add(egui::DragValue::new(value).clamp_range(range));
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                })
            }
        }
    };

    ($($num_type:ident),*) => {
        $(impl_for_num_types!($num_type);)*
    };
}

impl_for_num_types!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
