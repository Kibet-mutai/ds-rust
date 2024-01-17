#[allow(unused_imports)]
use std::{collections::HashMap, io::ErrorKind};

struct TextEditor<'a> {
    s: &'a str,
    stack: Vec<&'a str>,
}

impl<'a> TextEditor<'a> {
    pub fn new(data: &'a str) -> Self {
        TextEditor {
            s: data,
            stack: Vec::new(),
        }
    }
    pub fn append(&mut self) {
       self.stack.push(self.s);
    }

    pub fn del_char(&mut self, index: usize) -> &str {
        let index_t = self.s.chars().into_iter().count() as usize; 
        if index_t <= index {
            return "";
        }

        let mut indices = self.s.char_indices();

        let obtain_index = |(index, _char)| index;
        let str_len = self.s.len();

        unsafe {
            // SAFETY: Since `indices` iterates over the `CharIndices` of `self`, we can guarantee
            // that the indices obtained from it will always be within the bounds of `self` and they
            // will always lie on UTF-8 sequence boundaries.
            self.s.slice_unchecked(
                indices.nth(index).map_or(str_len, &obtain_index),
                indices
                    .nth(index_t - index - 1)
                    .map_or(str_len, &obtain_index),
            )
        }
    }

    pub fn print_char(&self, index: usize) {
        let new_s = self.s;
        let mut last_str = String::new();
        for (i, c) in new_s.chars().enumerate() {
             if i == index {
                last_str.push(c);
            }
        }
        println!("{}", last_str);
    }

    pub fn restore_string(&mut self) -> Option<&str> {
        self.stack.pop()
    }
}

fn text_editor(s: &str) {
    let mut stack: Vec<&str> = vec![];
    stack.push(s)
}

fn main() {
    let data = "foobar";
    let mut text_editor = TextEditor::new(data);
    let s = text_editor.s;
    let st = text_editor.append();
    //let stack = text_editor.stack;
    //println!("{:?}\n", stack);
    //let chare = text_editor.print_char(2);
    //let mut last = text_editor.del_char(1);
    println!("{:?}\n{}\n{:?}\n", st, s, text_editor.restore_string());
}
