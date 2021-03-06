use std::error::Error;
use std::io::{self, BufRead};

use algs4_rs::graphs::UnionFind;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let n = iter
        .next()
        .unwrap_or_else(|| Err(io::Error::new(io::ErrorKind::Other, "No line")))?
        .parse::<usize>()?;

    let mut uf = UnionFind::new(n);
    for line in iter {
        let line = line.unwrap();
        let mut parts = line.split_whitespace().map(|s| s.parse::<usize>());
        let (p, q) = match (parts.next(), parts.next()) {
            (Some(Ok(p)), Some(Ok(q))) => (p, q),

            _ => {
                return Err("Parse error".into());
            }
        };

        if let Some(true) = uf.connected(p, q) {
            continue;
        }

        uf.union(p, q);
        // println!("{} - {}", p, q);
    }
    println!("{} nodes within {} components", n, uf.count());

    Ok(())
}
