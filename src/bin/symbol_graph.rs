use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use algs4_rs::graphs::SymbolGraph;

struct EdgeReader {
    lines: io::Lines<io::BufReader<File>>,
    delimiter: String,
    stash: Vec<(String, String)>,
}
impl EdgeReader {
    pub fn new(file: File, delimiter: &str) -> Self {
        EdgeReader {
            lines: BufReader::new(file).lines(),
            delimiter: String::from(delimiter),
            stash: Vec::new(),
        }
    }
}
impl Iterator for EdgeReader{
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stash.len() > 0 {
            return self.stash.pop();
        }

        self.lines.next().map_or(None, |line| {
            line.ok().and_then(|line| {
                let mut parts = line.split(&self.delimiter);
                if let Some(v) = parts.next() {
                    for w in parts {
                        self.stash.push((v.into(), w.into()));
                    }
                }
                self.stash.pop()
            })
        })
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: <filename> [delimiter]".into());
    }
    let filename = &args[1];
    let delimiter = if args.len() > 2 { &args[2] } else { " " };

    let file = File::open(filename)?;

    let er = EdgeReader::new(file, delimiter);
    let sg = SymbolGraph::new(er);
    println!("V={},E={}", sg.v_size(), sg.e_size());

    println!("ready to query:");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if !sg.contains(&line) {
            println!("{} not exists!", line);
            continue;
        }

        for w in sg.adj(&line) {
            println!("\t{}", w);
        }
    }

    Ok(())
}
