use std::fs;
use std::io;

mod add_ticker;

struct WatchItem {
    ticker: String,
    threshold: f64,
}

fn load_watchlist() -> Vec<WatchItem> {
    let content = fs::read_to_string("watchlist.txt").unwrap_or_default();

    let mut items = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 2 {
            let ticker = parts[0].trim().to_string();
            let threshold = parts[1].trim().parse::<f64>().unwrap_or(0.0);

            items.push(WatchItem { ticker, threshold });
        }
    }

    items
}

fn main() {
    println!("\n=== Price Ping ===");
    println!("1. View Watchlist");
    println!("2. Add Stock");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            let watchlist = load_watchlist();

            if watchlist.is_empty() {
                println!("No stocks saved.");
            } else {
                for item in watchlist {
                    println!("{} -> Alert at ${}", item.ticker, item.threshold);
                }
            }
        }
        "2" => {
            add_ticker::add_stock();
        }
        _ => {
            println!("Invalid option");
        }
    }
}
