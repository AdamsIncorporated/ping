use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn add_stock() {
    let mut ticker = String::new();
    let mut threshold = String::new();

    println!("Enter ticker:");
    io::stdin().read_line(&mut ticker).unwrap();

    println!("Enter threshold:");
    io::stdin().read_line(&mut threshold).unwrap();

    let line = format!("{},{}\n", ticker.trim().to_uppercase(), threshold.trim());

    OpenOptions::new()
        .append(true)
        .create(true)
        .open("watchlist.txt")
        .unwrap()
        .write_all(line.as_bytes())
        .unwrap();

    println!("✅ Saved!");
}
