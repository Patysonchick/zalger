use crate::smbls::Depth::Smlr;
use std::cmp::PartialEq;
use std::collections::HashMap;

const ZWSP: char = '\u{200B}'; // пробел нулевой ширины

pub struct Input {
    text: String,
    depth: Depth,
}

#[derive(Default, Debug, PartialEq)]
enum Depth {
    #[default]
    Same,
    Smlr,
}

impl Input {
    pub fn new(text: String) -> Self {
        Input {
            text: text.trim().to_string(),
            depth: Default::default(),
        }
    }

    pub fn smlr_depth(&mut self) {
        self.depth = Smlr;
    }

    pub fn obfs(&self) -> String {
        let mut out = String::new();

        for c in self.text.chars() {
            let swaped = self.swap_symbol(c);
            out.push(swaped);
            out.push(ZWSP);
        }

        out
    }

    fn swap_symbol(&self, c: char) -> char {
        let mut smbls = get_smbls_same();
        if self.depth == Smlr {
            smbls.extend(get_smbls_smlr());
        }

        smbls.get(&c).cloned().unwrap_or(c)
    }
}

pub fn get_smbls_same() -> HashMap<char, char> {
    let mut smbls_same = HashMap::new();
    smbls_same.insert('А', 'A');
    smbls_same.insert('В', 'B');
    smbls_same.insert('Е', 'E');
    smbls_same.insert('К', 'K');
    smbls_same.insert('М', 'M');
    smbls_same.insert('Н', 'H');
    smbls_same.insert('О', 'O');
    smbls_same.insert('Р', 'P');
    smbls_same.insert('С', 'C');
    smbls_same.insert('Т', 'T');
    smbls_same.insert('Х', 'X');

    smbls_same.insert('а', 'a');
    smbls_same.insert('е', 'e');
    smbls_same.insert('о', 'o');
    smbls_same.insert('р', 'p');
    smbls_same.insert('с', 'c');
    smbls_same.insert('у', 'y');
    smbls_same.insert('х', 'x');

    smbls_same
}

pub fn get_smbls_smlr() -> HashMap<char, char> {
    let mut smbls_smlr = HashMap::new();
    smbls_smlr.insert('З', '3');
    smbls_smlr.insert('з', '3');
    smbls_smlr.insert('о', 'ο');
    smbls_smlr.insert('а', 'α');
    smbls_smlr.insert('р', 'ρ');
    smbls_smlr.insert('с', 'ѕ');

    smbls_smlr.insert('і', 'i');
    smbls_smlr.insert('І', 'I');

    smbls_smlr
}
