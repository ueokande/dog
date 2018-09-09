use std::io::{BufRead, BufReader, Result};

trait LineFilter {
    fn filter(&self, line: String) -> String;
}

struct DogLineFilter;

enum CharType {
    B,
    O,
    W,
}

impl CharType {
    fn next(&self) -> Self {
        use CharType::*;
        match self {
            B => O,
            O => W,
            W => O,
        }
    }
}

impl CharType {
    fn ch(&self) -> char {
        use CharType::*;
        match self {
            B => 'B',
            O => 'o',
            W => 'w',
        }
    }
}

impl LineFilter for DogLineFilter {
    fn filter(&self, line: String) -> String {
        let mut bowow = String::with_capacity(line.len());
        let mut ct = CharType::B;
        for c in line.chars() {
            if c.is_whitespace() {
                bowow.push(c);
                ct = CharType::B
            } else {
                bowow.push(ct.ch());
                ct = ct.next()
            }
        }
        bowow
    }
}

fn run(filename: String) -> Result<()> {
    let ref filter = &DogLineFilter;
    let f = std::fs::File::open(filename)?;
    for line in BufReader::new(f).lines() {
        let filtered = filter.filter(line?);
        println!("{}", filtered)
    }
    Ok(())
}

fn main() {
    let filename: String = match std::env::args().nth(1) {
        Some(x) => x,
        None => "/dev/stdin".to_string(),
    };

    if let Err(e) = run(filename.to_string()) {
        eprintln!("{}: {}", filename, e)
    }
}
