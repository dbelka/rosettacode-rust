//! ## The classic hash join algorithm for an inner join
//! http://rosettacode.org/wiki/Hash_join#Rust

use std::collections::HashMap;

fn main() {
    let table_a = vec![
        (27, "Jonah"), (18, "Alan"), (28, "Glory"), 
        (18, "Popeye"), (28, "Alan")
    ];
    let table_b = vec![
        ("Jonah", "Whales"), ("Jonah", "Spiders"), ("Alan", "Ghosts"), 
        ("Alan", "Zombies"), ("Glory", "Buffy")
    ];
    // hash phase
    let mut h = HashMap::new();
    for (i, a) in table_a.iter().enumerate() {
        h.entry(a.1).or_insert(vec![]).push(i);
    }
    // join phase
    for b in table_b {
        for i in h.get(b.0).unwrap_or(&vec![]) {
            let a = table_a.get(*i).unwrap();
            println!("{:?} {:?}", a, b);
        }
    }
}