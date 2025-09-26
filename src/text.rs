use std::{any::TypeId, ops::Range};

use crate::{option::option_probe_with, EguiProbe, Style};

impl EguiProbe for String {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::singleline(self))
    }
}

impl EguiProbe for &str {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::singleline(self))
    }
}

struct CharBuffer {
    ch: char,
    buf: [u8; 4],
}

impl CharBuffer {
    const fn new(ch: char) -> Self {
        let mut buf = [0; 4];
        let _ = ch.encode_utf8(&mut buf);
        CharBuffer { ch, buf }
    }
}

impl Default for CharBuffer {
    fn default() -> Self {
        Self::new('\0')
    }
}

impl egui::text_edit::TextBuffer for CharBuffer {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        // SAFETY: prefix of buf is valid UTF-8
        unsafe { str::from_utf8_unchecked(&self.buf[..self.ch.len_utf8()]) }
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        if char_index > 1 {
            return 0;
        }
        match text.chars().next() {
            None => 0,
            Some(c) => {
                self.ch = c;
                let _ = c.encode_utf8(&mut self.buf);
                1
            }
        }
    }

    fn delete_char_range(&mut self, _char_range: Range<usize>) {}

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl EguiProbe for char {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        let mut buf = CharBuffer::new(*self);
        let r = ui.add(egui::TextEdit::singleline(&mut buf));
        if r.changed() {
            *self = buf.ch;
        }
        r
    }
}

/// Wrapper for string-like types to show multiline text field.
pub struct EguiProbeMultiline<'a, T> {
    pub string: &'a mut T,
}

impl EguiProbe for EguiProbeMultiline<'_, String> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::multiline(self.string))
    }
}

impl EguiProbe for EguiProbeMultiline<'_, &str> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::multiline(self.string))
    }
}

impl EguiProbe for EguiProbeMultiline<'_, Option<String>> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        option_probe_with(self.string, ui, style, String::new, |string, ui, _| {
            ui.add(egui::TextEdit::multiline(string))
        })
    }
}

impl EguiProbe for EguiProbeMultiline<'_, Option<&str>> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        option_probe_with(
            self.string,
            ui,
            style,
            || "",
            |string, ui, _| ui.add(egui::TextEdit::multiline(string)),
        )
    }
}
