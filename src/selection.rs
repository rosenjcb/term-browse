use crossterm::{Terminal, terminal, TerminalCursor, cursor, Color};
use crate::screen::Screen;
use crate::coord::Coord;

pub struct Selection<'a, 'b>{
    context: &'a Screen,
    content: (&'b Coord, &'b Coord),
    cursor: TerminalCursor,
}

impl<'a, 'b> Selection<'a, 'b> {
    pub fn new(&self, context: &mut Screen) -> Self {
        let coord = Coord::new(0,0);
        let content = (&coord, &coord);
        let cursor = cursor();
        Selection { context, content , cursor}
    }
    pub fn start(&mut self, coord: (u16, u16)) {
        let start = Coord::new(coord.0, coord.1);
        self.content.0 = &start;
        self.content.1 = &start;
        //self.context = context;
    }
    pub fn update(&mut self, end: (u16,u16)) {
        let end = Coord::new(end.0, end.1);
        //let paint = paint(self.context.buffer[end.1][end.0].c.with(Color::Black).on(Color::Yellow));
        self.content.1 = &end;
       // self.context
    }

    pub fn copy() {

    }

    pub fn paste() {

    }

    pub fn cut() {

    }
}