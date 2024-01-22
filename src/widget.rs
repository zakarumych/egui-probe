use core::hash::Hash;

use crate::{EguiProbe, Style};

#[derive(Clone, Copy)]
struct ProbeHeaderState {
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
        let state = cx.data_mut(|d| {
            *d.get_temp_mut_or(
                id,
                ProbeHeaderState {
                    open: false,
                    body_height: 0.0,
                },
            )
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

    fn toggle(&mut self) {
        self.state.open = !self.state.open;
        self.dirty = true;
    }

    // fn is_open(&self) -> bool {
    //     self.state.open
    // }

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
    dirty: bool,
    min_labels_width: f32,
}

impl ProbeLayout {
    fn load(cx: &egui::Context, id: egui::Id) -> ProbeLayout {
        let state = cx.data_mut(|d| *d.get_temp_mut_or(id, ProbeLayoutState { labels_width: 0.0 }));
        ProbeLayout {
            id,
            state,
            dirty: false,
            min_labels_width: 0.0,
        }
    }

    fn store(mut self, cx: &egui::Context) {
        if self.dirty {
            self.state.labels_width = self.min_labels_width;
            cx.data_mut(|d| d.insert_temp(self.id, self.state));
            cx.request_repaint();
        }
    }

    fn bump_labels_width(&mut self, width: f32) {
        if self.min_labels_width < width {
            self.min_labels_width = width;
            self.dirty = true;
        }
    }

    pub fn inner_label_ui(
        &mut self,
        indent: usize,
        id_source: impl Hash,
        ui: &mut egui::Ui,
        add_content: impl FnOnce(&mut egui::Ui),
    ) {
        let labels_width = self.state.labels_width;
        let cursor = ui.cursor();

        let max = egui::pos2(cursor.max.x.min(cursor.min.x + labels_width), cursor.max.y);
        let min = egui::pos2(cursor.min.x, cursor.min.y);
        let rect = egui::Rect::from_min_max(min, max);

        let mut label_ui =
            ui.child_ui_with_id_source(rect.intersect(ui.max_rect()), *ui.layout(), id_source);
        label_ui.set_clip_rect(
            ui.clip_rect()
                .intersect(egui::Rect::everything_left_of(max.x)),
        );

        for _ in 0..indent {
            label_ui.separator();
        }

        add_content(&mut label_ui);
        let mut final_rect = label_ui.min_rect();

        self.bump_labels_width(final_rect.width());

        final_rect.max.x = final_rect.min.x + labels_width;

        ui.advance_cursor_after_rect(final_rect);
    }

    pub fn inner_value_ui(
        &mut self,
        id_source: impl Hash,
        ui: &mut egui::Ui,
        add_content: impl FnOnce(&mut egui::Ui),
    ) {
        let mut value_ui = ui.child_ui_with_id_source(
            ui.cursor().intersect(ui.max_rect()),
            *ui.layout(),
            id_source,
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
pub struct Probe<'a, T> {
    id_source: egui::Id,
    label: egui::WidgetText,
    style: Style,
    value: &'a mut T,
}

impl<'a, T> Probe<'a, T>
where
    T: EguiProbe,
{
    /// Creates a new `Probe` widget.
    pub fn new(label: impl Into<egui::WidgetText>, value: &'a mut T) -> Self {
        let label = label.into();
        Probe {
            id_source: egui::Id::new(label.text()),
            label,
            style: Style::default(),
            value,
        }
    }

    /// Show probbing UI to edit the value.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        if !self.value.has_inner() {
            return self.value.probe(ui, &self.style);
        }

        ui.allocate_ui(ui.available_size(), |ui| {
            let ref mut child_ui = ui.child_ui_with_id_source(
                ui.max_rect(),
                egui::Layout::top_down(egui::Align::Min),
                self.id_source,
            );

            let mut header =
                ProbeHeader::load(child_ui.ctx(), child_ui.make_persistent_id("probe_header"));

            egui::Frame::none()
                .fill(child_ui.visuals().extreme_bg_color)
                .inner_margin(child_ui.spacing().item_spacing * 0.5)
                .show(child_ui, |child_ui| {
                    child_ui.horizontal(|child_ui| {
                        header.collapse_button(child_ui);
                        child_ui.label(self.label);
                    });
                });

            if header.openness > 0.0 && self.value.has_inner() {
                let mut layout =
                    ProbeLayout::load(child_ui.ctx(), child_ui.make_persistent_id("probe_layout"));

                show_table(
                    self.value,
                    &mut header,
                    &mut layout,
                    0,
                    child_ui,
                    &self.style,
                    "table",
                );

                layout.store(child_ui.ctx());
            }

            header.store(child_ui.ctx());

            let final_rect = child_ui.min_rect();
            ui.advance_cursor_after_rect(final_rect);

            // let response = ui.interact(final_rect, child_ui.id(), egui::Sense::hover());
            // response.widget_info(|| egui::WidgetInfo::new(egui::WidgetType::Other));

            // response
        })
        .response
    }
}

fn show_header(
    label: &str,
    value: &mut dyn EguiProbe,
    layout: &mut ProbeLayout,
    indent: usize,
    ui: &mut egui::Ui,
    style: &Style,
    id_source: impl Hash,
) -> Option<ProbeHeader> {
    let mut header = None;

    let id = ui.make_persistent_id(id_source);

    if value.has_inner() {
        header = Some(ProbeHeader::load(ui.ctx(), id));
    }

    ui.horizontal(|ui| {
        layout.inner_label_ui(indent, id.with("label"), ui, |ui| {
            if let Some(header) = &mut header {
                header.collapse_button(ui);
            }
            ui.label(label);
        });

        layout.inner_value_ui(id.with("value"), ui, |ui| {
            value.probe(ui, style);
        });
    });

    header
}

fn show_table(
    value: &mut dyn EguiProbe,
    header: &mut ProbeHeader,
    layout: &mut ProbeLayout,
    indent: usize,
    ui: &mut egui::Ui,
    style: &Style,
    id_source: impl Hash,
) {
    let cursor = ui.cursor();

    let table_rect = egui::Rect::from_min_max(
        egui::pos2(cursor.min.x, cursor.min.y - header.body_shift()),
        ui.max_rect().max,
    );

    let mut table_ui = ui.child_ui_with_id_source(
        table_rect,
        egui::Layout::top_down(egui::Align::Min),
        id_source,
    );
    table_ui.set_clip_rect(
        ui.clip_rect()
            .intersect(egui::Rect::everything_below(ui.min_rect().max.y)),
    );

    let mut idx = 0;
    value.iterate_inner(&mut |label, value| {
        let header = show_header(label, value, layout, indent + 1, &mut table_ui, style, idx);

        if let Some(mut header) = header {
            if header.openness > 0.0 {
                show_table(
                    value,
                    &mut header,
                    layout,
                    indent + 1,
                    &mut table_ui,
                    style,
                    idx,
                );
            }
            header.store(table_ui.ctx());
        }

        idx += 1;
    });

    let final_table_rect = table_ui.min_rect();

    ui.advance_cursor_after_rect(final_table_rect);
    let table_height = ui.cursor().min.y - table_rect.min.y;
    header.set_body_height(table_height);
}
