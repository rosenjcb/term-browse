use crate::gridrow::GridRow;
use crossterm::*;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::fs;
use std::path::Path;

pub struct Screen {
    pub buffer: Vec<GridRow>,
    pub view_loc: u16,
    terminal: Terminal,
}

impl Screen{
    pub fn new(x: i32, y: i32) -> Self {
        //let mut buffer: Vec<GridRow> = Vec::with_capacity(usize::from(y));
        let mut buffer = (0 .. y).map(|_| GridRow::new(x)).collect::<Vec<_>>();
        /*for i in 0..y {
            buffer.push(GridRow::new(x as i32));
        }*/
        let view_loc = 0;
        let terminal = terminal();
        Screen{ buffer, view_loc, terminal }
    }

    pub fn save(&self){
        let file = File::create("test.txt").expect("Couldn't create file.");
        let mut filewriter = LineWriter::new(file);
        self.buffer.iter().for_each(|row| {
            filewriter.write(row.getline().as_bytes()).expect("Cannot write to buffer");
        });

    }

    pub fn write(&mut self, x: u16, y: u16, c: char) {
        self.terminal.write(c);
        self.buffer[y as usize].write(x as i32, c);
    }

    pub fn load(&mut self, filepath: &Path) {
        let cursor = cursor();
        let contents = fs::read_to_string(filepath).expect("File not found!");
        cursor.goto(0,0);
        println!("{}", contents);
    }

    //This sucks...
    /*pub fn render(&self) {
        //(0..self.buffer.len()).for_each(|line| self.buffer[line].printline());
        for(index, line) in self.buffer.iter().enumerate() {
            line.printline(self.view_loc + (index as u16));
        }
    }*/
}