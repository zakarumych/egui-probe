use core::hash::Hash;

use egui::WidgetText;

use crate::{EguiProbe, Style};

#[derive(Clone, Copy)]
struct ProbeHeaderState {
    has_inner: bool,
    open: bool,
    body_height: f32,
}

struct ProbeHeader {
    id: egui::Id,
    state: ProbeHeaderState,
    dirty: bool,
    openness: f32,
}

impl ProbeHeader {
    fn load(cx: &egui::Context, id: egui::Id) -> ProbeHeader {
        let state = cx.data_mut(|d| d.get_temp(id)).unwrap_or(ProbeHeaderState {
            has_inner: false,
            open: false,
            body_height: 0.0,
        });

        let openness = cx.animate_bool(id, state.open);

        ProbeHeader {
            id,
            state,
            dirty: false,
            openness,
        }
    }

    fn store(self, cx: &egui::Context) {
        if self.dirty {
            cx.data_mut(|d| d.insert_temp(self.id, self.state));
            cx.request_repaint();
        }
    }

    pub const fn has_inner(&self) -> bool {
        self.state.has_inner
    }

    pub const fn set_has_inner(&mut self, has_inner: bool) {
        if self.state.has_inner != has_inner {
            self.state.has_inner = has_inner;
            self.dirty = true;
        }
    }

    const fn toggle(&mut self) {
        self.state.open = !self.state.open;
        self.dirty = true;
    }

    fn set_body_height(&mut self, height: f32) {
        // TODO: Better approximation
        if (self.state.body_height - height).abs() > 0.001 {
            self.state.body_height = height;
            self.dirty = true;
        }
    }

    fn body_shift(&self) -> f32 {
        (1.0 - self.openness) * self.state.body_height
    }

    fn collapse_button(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = ui.spacing().icon_width_inner;
        let response =
            ui.allocate_response(egui::vec2(desired_size, desired_size), egui::Sense::click());

        if response.clicked() {
            self.toggle();
        }

        egui::collapsing_header::paint_default_icon(ui, self.openness, &response);
        response
    }
}

#[derive(Clone, Copy)]
struct ProbeLayoutState {
    labels_width: f32,
}

pub struct ProbeLayout {
    id: egui::Id,
    state: ProbeLayoutState,
    min_labels_width: f32,
}

impl ProbeLayout {
    fn load(cx: &egui::Context, id: egui::Id) -> ProbeLayout {
        let state = cx.data_mut(|d| *d.get_temp_mut_or(id, ProbeLayoutState { labels_width: 0.0 }));
        ProbeLayout {
            id,
            state,
            min_labels_width: 0.0,
        }
    }

    fn store(mut self, cx: &egui::Context) {
        if self.state.labels_width != self.min_labels_width {
            self.state.labels_width = self.min_labels_width;
            cx.data_mut(|d| d.insert_temp(self.id, self.state));
            cx.request_repaint();
        }
    }

    fn bump_labels_width(&mut self, width: f32) {
        if self.min_labels_width < width {
            self.min_labels_width = width;
        }
    }

    pub fn inner_label_ui(
        &mut self,
        indent: usize,
        id_salt: impl Hash,
        ui: &mut egui::Ui,
        add_content: impl FnOnce(&mut egui::Ui) -> egui::Response,
    ) -> egui::Response {
        let labels_width = self.state.labels_width;
        let cursor = ui.cursor();

        let max = egui::pos2(cursor.max.x.min(cursor.min.x + labels_width), cursor.max.y);
        let min = egui::pos2(cursor.min.x, cursor.min.y);
        let rect = egui::Rect::from_min_max(min, max);

        let mut label_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect.intersect(ui.max_rect()))
                .layout(*ui.layout())
                .id_salt(id_salt),
        );
        label_ui.set_clip_rect(
            ui.clip_rect()
                .intersect(egui::Rect::everything_left_of(max.x)),
        );

        for _ in 0..indent {
            label_ui.separator();
        }

        let label_response = add_content(&mut label_ui);
        let mut final_rect = label_ui.min_rect();

        self.bump_labels_width(final_rect.width());

        final_rect.max.x = final_rect.min.x + labels_width;

        ui.advance_cursor_after_rect(final_rect);
        label_response
    }

    pub fn inner_value_ui(
        &mut self,
        id_salt: impl Hash,
        ui: &mut egui::Ui,
        add_content: impl FnOnce(&mut egui::Ui),
    ) {
        let mut value_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(ui.cursor().intersect(ui.max_rect()))
                .layout(*ui.layout())
                .id_salt(id_salt),
        );

        add_content(&mut value_ui);
        let final_rect = value_ui.min_rect();
        ui.advance_cursor_after_rect(final_rect);
    }
}

/// Widget for editing a value via `EguiProbe` trait.
///
/// For simple values it will show a probe UI for it.
/// For complex values it will header with collapsible body.
#[must_use = "You should call .show()"]
pub struct Probe<'a> {
    header: Option<egui::WidgetText>,
    style: Style,
    value: &'a mut dyn EguiProbe,
}

impl<'a> Probe<'a> {
    /// Creates a new `Probe` widget.
    pub fn new(value: &'a mut dyn EguiProbe) -> Self {
        Probe {
            // id_salt: egui::Id::new(label.text()),
            header: None,
            style: Style::default(),
            value,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_header(mut self, label: impl Into<WidgetText>) -> Self {
        self.header = Some(label.into());
        self
    }

    /// Show probbing UI to edit the value.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let mut changed = false;

        let mut r = ui
            .allocate_ui(ui.available_size(), |ui| {
                let child_ui = &mut ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(ui.max_rect())
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );

                let id = child_ui.next_auto_id();

                let mut layout = ProbeLayout::load(child_ui.ctx(), id);

                if let Some(label) = self.header {
                    let mut header = show_header(
                        label,
                        self.value,
                        &mut layout,
                        0,
                        child_ui,
                        &self.style,
                        id,
                        &mut changed,
                    );

                    if header.openness > 0.0 {
                        show_table(
                            self.value,
                            &mut header,
                            &mut layout,
                            0,
                            child_ui,
                            &self.style,
                            id,
                            &mut changed,
                        );
                    } else {
                        let mut got_inner = false;

                        self.value.iterate_inner(ui, &mut |_, _, _| {
                            got_inner = true;
                        });

                        header.set_has_inner(got_inner);
                    }

                    header.store(child_ui.ctx());
                } else {
                    show_table_direct(
                        self.value,
                        &mut layout,
                        0,
                        child_ui,
                        &self.style,
                        id,
                        &mut changed,
                    );
                }

                layout.store(child_ui.ctx());

                let final_rect = child_ui.min_rect();
                ui.advance_cursor_after_rect(final_rect);

                // let response = ui.interact(final_rect, child_ui.id(), egui::Sense::hover());
                // response.widget_info(|| egui::WidgetInfo::new(egui::WidgetType::Other));

                // response
            })
            .response;

        if changed {
            r.mark_changed();
        }

        r
    }
}

#[allow(clippy::too_many_arguments)]
fn show_header(
    label: impl Into<WidgetText>,
    value: &mut dyn EguiProbe,
    layout: &mut ProbeLayout,
    indent: usize,
    ui: &mut egui::Ui,
    style: &Style,
    id_salt: impl Hash,
    changed: &mut bool,
) -> ProbeHeader {
    let id = ui.make_persistent_id(id_salt);

    let mut header = ProbeHeader::load(ui.ctx(), id);

    ui.horizontal(|ui| {
        let label_response = layout.inner_label_ui(indent, id.with("label"), ui, |ui| {
            if header.has_inner() {
                header.collapse_button(ui);
            }
            ui.label(label)
        });

        layout.inner_value_ui(id.with("value"), ui, |ui| {
            *changed |= value
                .probe(ui, style)
                .labelled_by(label_response.id)
                .changed();
        });
    });

    header
}

#[allow(clippy::too_many_arguments)]
fn show_table(
    value: &mut dyn EguiProbe,
    header: &mut ProbeHeader,
    layout: &mut ProbeLayout,
    indent: usize,
    ui: &mut egui::Ui,
    style: &Style,
    id_salt: impl Hash,
    changed: &mut bool,
) {
    let cursor = ui.cursor();

    let table_rect = egui::Rect::from_min_max(
        egui::pos2(cursor.min.x, cursor.min.y - header.body_shift()),
        ui.max_rect().max,
    );

    let mut table_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(table_rect)
            .layout(egui::Layout::top_down(egui::Align::Min))
            .id_salt(id_salt),
    );
    table_ui.set_clip_rect(
        ui.clip_rect()
            .intersect(egui::Rect::everything_below(ui.min_rect().max.y)),
    );

    let mut got_inner = false;
    let mut idx = 0;
    value.iterate_inner(&mut table_ui, &mut |label, table_ui, value| {
        got_inner = true;

        let mut header = show_header(
            label,
            value,
            layout,
            indent + 1,
            table_ui,
            style,
            idx,
            changed,
        );

        if header.openness > 0.0 {
            show_table(
                value,
                &mut header,
                layout,
                indent + 1,
                table_ui,
                style,
                idx,
                changed,
            );
        } else {
            let mut got_inner = false;

            value.iterate_inner(ui, &mut |_, _, _| {
                got_inner = true;
            });

            header.set_has_inner(got_inner);
        }

        header.store(table_ui.ctx());

        idx += 1;
    });

    header.set_has_inner(got_inner);

    let final_table_rect = table_ui.min_rect();

    ui.advance_cursor_after_rect(final_table_rect);
    let table_height = ui.cursor().min.y - table_rect.min.y;
    header.set_body_height(table_height);
}

fn show_table_direct(
    value: &mut dyn EguiProbe,
    layout: &mut ProbeLayout,
    indent: usize,
    ui: &mut egui::Ui,
    style: &Style,
    id_salt: impl Hash,
    changed: &mut bool,
) {
    let cursor = ui.cursor();

    let table_rect =
        egui::Rect::from_min_max(egui::pos2(cursor.min.x, cursor.min.y), ui.max_rect().max);

    let mut table_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(table_rect)
            .layout(egui::Layout::top_down(egui::Align::Min))
            .id_salt(id_salt),
    );
    table_ui.set_clip_rect(
        ui.clip_rect()
            .intersect(egui::Rect::everything_below(ui.min_rect().max.y)),
    );

    let mut got_inner = false;
    let mut idx = 0;
    value.iterate_inner(&mut table_ui, &mut |label, table_ui, value| {
        got_inner = true;

        let mut header = show_header(
            label,
            value,
            layout,
            indent + 1,
            table_ui,
            style,
            idx,
            changed,
        );

        if header.openness > 0.0 {
            show_table(
                value,
                &mut header,
                layout,
                indent + 1,
                table_ui,
                style,
                idx,
                changed,
            );
        } else {
            let mut got_inner = false;

            value.iterate_inner(ui, &mut |_, _, _| {
                got_inner = true;
            });

            header.set_has_inner(got_inner);
        }

        header.store(table_ui.ctx());

        idx += 1;
    });

    let final_table_rect = table_ui.min_rect();
    ui.advance_cursor_after_rect(final_table_rect);
}
