use crate::gridcell::{GridCell, Color};
use crossterm::{Terminal, terminal, TerminalCursor, cursor};

pub struct GridRow {
    pub inner: Vec<GridCell>,
    terminal: Terminal,
    cursor: TerminalCursor,
}

impl GridRow {
    pub fn new(columns: i32) -> Self {
        let mut inner: Vec<GridCell> = Vec::with_capacity(columns as usize);
        let terminal = terminal();
        let cursor = cursor();
        //(0 .. columns).map(|_| GridRow::new(x)).collect::<Vec<_>>()
        for i in 0..columns {
            let cell: GridCell = GridCell{c: ' ', fg: Color::WHITE, bg: Color::BLACK};
            inner.push(cell);
        }
        GridRow { inner, terminal, cursor }
    }

    pub fn write(&mut self, cell_index: i32, c: char) {
        self.inner[cell_index as usize].c = c;
    }

    //This sucks...
    pub fn printline(&self, line_number: u16) {
        for (index, cell) in self.inner.iter().enumerate() {
            self.cursor.goto(index as u16, line_number);
            self.terminal.write(cell.c);
        }
    }

    pub fn getline(&self) -> String {
        let s: String = self.inner.iter().map(|cell| cell.c).collect();
        return s;
    }
}