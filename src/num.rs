use core::ops::{RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

use egui::emath::Numeric;

use crate::{option::option_probe_with, EguiProbe, Style};

/// Marker type to indicate that the step for range is not set.
pub struct StepUnset;

/// Bundles value and a range to show probbing UI to edit the value in that range.
pub struct EguiProbeRange<'a, T, R, S = StepUnset> {
    pub value: &'a mut T,
    pub range: R,
    pub step: S,
}

pub fn non_negative<N: Numeric>(value: &mut N) -> EguiProbeRange<'_, N, RangeFrom<N>> {
    EguiProbeRange {
        value,
        range: N::from_f64(0.0)..,
        step: StepUnset,
    }
}

// pub fn non_negative_with_step<N: Numeric, S>(
//     value: &mut N,
//     step: S,
// ) -> EguiProbeRange<'_, N, RangeFrom<N>, S> {
//     EguiProbeRange {
//         value,
//         range: N::from_f64(0.0)..,
//         step,
//     }
// }

// pub fn range_from<'a, T>(value: &'a mut T, from: T) -> EguiProbeRange<'a, T, RangeFrom<T>> {
//     EguiProbeRange {
//         value,
//         range: from..,
//     }
// }

// pub fn range_to<'a, T>(value: &'a mut T, to: T) -> EguiProbeRange<'a, T, RangeToInclusive<T>> {
//     EguiProbeRange {
//         value,
//         range: ..=to,
//     }
// }

// pub fn range<'a, T>(
//     value: &'a mut T,
//     range: RangeInclusive<T>,
// ) -> EguiProbeRange<'a, T, RangeInclusive<T>> {
//     EguiProbeRange { value, range }
// }

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
                ui.add(egui::DragValue::new(self.value).range(range))
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeFrom<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).range(range)).changed();
                    ui.weak(format!("{}..", self.range.start));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeToInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).range(range)).changed();
                    ui.weak(format!("..={}", self.range.end));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.clone();
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).range(range)).changed();
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFull> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=$num_type::MAX;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    ui.add(egui::DragValue::new(value).range(range))
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFrom<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).range(range));
                    ui.weak(format!("{}..", self.range.start));
                    r
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeToInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).range(range));
                    ui.weak(format!("..={}", self.range.end));
                    r
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.clone();
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).range(range));
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                    r
                })
            }
        }







        impl<S> EguiProbe for EguiProbeRange<'_, $num_type, RangeFull, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = $num_type::MIN..=$num_type::MAX;
                ui.add(egui::DragValue::new(self.value).range(range).speed(self.step.into()))
            }
        }

        impl<S> EguiProbe for EguiProbeRange<'_, $num_type, RangeFrom<$num_type>, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).range(range).speed(self.step.into())).changed();
                    ui.weak(format!("{}..", self.range.start));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl<S> EguiProbe for EguiProbeRange<'_, $num_type, RangeToInclusive<$num_type>, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).range(range).speed(self.step.into())).changed();
                    ui.weak(format!("..={}", self.range.end));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl<S> EguiProbe for EguiProbeRange<'_, $num_type, RangeInclusive<$num_type>, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.clone();
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).range(range).speed(self.step.into())).changed();
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl<S> EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFull, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=$num_type::MAX;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    ui.add(egui::DragValue::new(value).range(range).speed(self.step.into()))
                })
            }
        }

        impl<S> EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFrom<$num_type>, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).range(range).speed(self.step.into()));
                    ui.weak(format!("{}..", self.range.start));
                    r
                })
            }
        }

        impl<S> EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeToInclusive<$num_type>, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).range(range).speed(self.step.into()));
                    ui.weak(format!("..={}", self.range.end));
                    r
                })
            }
        }

        impl<S> EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeInclusive<$num_type>, S> where S: Copy + Into<f64> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.clone();
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).range(range).speed(self.step.into()));
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                    r
                })
            }
        }
    };

    ($($num_type:ident),*) => {
        $(impl_for_num_types!($num_type);)*
    };
}

impl_for_num_types!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
