use crossterm::{Terminal, terminal, TerminalCursor, cursor, Color};
use crate::screen::Screen;
use crate::coord::Coord;

pub struct Selection {
    cursor: TerminalCursor,
    content: String,
    clipboard: String,
}

impl Selection {
    pub fn new() -> Self {
        let cursor = cursor();
        let (content, clipboard) = (String::from(""), String::from(""));
        Selection { cursor, content, clipboard }
    }
    pub fn start(&mut self, coord: (u16, u16)) {
        /*self.content.0.x = coord.0;
        self.content.0.y = coord.1;
        self.content.1.x = coord.0;
        self.content.1.y = coord.1;*/
        //self.context = context;
    }
    pub fn update(&mut self, end: (u16,u16)) {
        //let paint = paint(self.context.buffer[end.1][end.0].c.with(Color::Black).on(Color::Yellow));
        /*self.content.1.x = end.0;
        self.content.1.y = end.1;*/
    }

    pub fn copy(&mut self) {
        self.clipboard = String::from(self.content.as_str());
    }

    pub fn paste() {

    }

    pub fn cut() {

    }
}