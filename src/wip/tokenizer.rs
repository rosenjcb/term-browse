use std::{fs, thread, io};
use std::io::Write;
//use termion::{color, clear};
use std::time::Duration;
use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::{CharacterTokens, EndTag, NullCharacterToken, StartTag, TagToken};
use html5ever::tokenizer::{
    ParseError, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};

#[derive(Copy, Clone)]
struct TokenPrinter {
    in_char_run: bool,
}

impl TokenPrinter {
    fn is_char(&mut self, is_char: bool) {
        match (self.in_char_run, is_char) {
            (false, true) => print!("CHAR : \""),
            (true, false) => println!("\""),
            _ => (),
        }
        self.in_char_run = is_char;
    }

    fn do_char(&mut self, c: char) {
        self.is_char(true);
        print!("{}", c.escape_default().collect::<String>());
    }
}

impl TokenSink for TokenPrinter {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            CharacterTokens(b) => {
                for c in b.chars() {
                    self.do_char(c);
                }
            },
            NullCharacterToken => self.do_char('\0'),
            TagToken(tag) => {
                self.is_char(false);
                // This is not proper HTML serialization, of course.
                match tag.kind {
                    StartTag => print!("TAG  : <\x1b[32m{}\x1b[0m", tag.name),
                    EndTag => print!("TAG  : <\x1b[31m/{}\x1b[0m", tag.name),
                }
                for attr in tag.attrs.iter() {
                    print!(
                        " \x1b[36m{}\x1b[0m='\x1b[34m{}\x1b[0m'",
                        attr.name.local, attr.value
                    );
                }
                if tag.self_closing {
                    print!(" \x1b[31m/\x1b[0m");
                }
                println!(">");
            },
            ParseError(err) => {
                self.is_char(false);
                println!("ERROR: {}", err);
            },
            _ => {
                self.is_char(false);
                println!("OTHER: {:?}", token);
            },
        }
        TokenSinkResult::Continue
    }
}

fn main(){
    let mut sink = TokenPrinter { in_char_run: false };
    let mut chunk = ByteTendril::new();
    io::stdin().read_to_tendril(&mut chunk).unwrap();
    let mut input = BufferQueue::new();
    input.push_back(chunk.try_reinterpret().unwrap());

    let mut tok = Tokenizer::new(
        sink,
        TokenizerOpts {
            profile: true,
            ..Default::default()
        },
    );
    let _ = tok.feed(&mut input);
    //
    // assert!(input.is_empty());
    tok.end();
    sink.is_char(false);
}
