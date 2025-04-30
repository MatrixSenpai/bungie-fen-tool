#![allow(unused, dead_code)]

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct Data {
    sequence: usize,
    symbols: Vec<Vec<Option<String>>>,
    fill: String,
    has_qr: Option<bool>,
}

fn main() {
    let cache: HashMap<String, Data> = reqwest::blocking::get("https://tjl.co/queens-gambit-arg/data.json").unwrap().json().unwrap();

    loop {
        let mut buffer = String::new();
        println!("Enter freq (0 to quit) >");
        std::io::stdin().read_line(&mut buffer).unwrap();

        println!("Checking input {buffer}");
        let v = match buffer.trim().parse::<usize>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Not a number! {e:?}");
                continue;
            }
        };

        if v < 1000 || v > 9999 {
            eprintln!("Invalid frequency size! Enter between 1000-9999");
            continue
        }
        if v == 0 { std::process::exit(0); }

        let item = cache.iter().find(|(_, val)| {
            val.sequence.eq(&v)
        });
        if item.is_none() {
            eprintln!("No item found for that frequency!");
            continue
        }

        let mut fen = String::new();
        let mut fskip = 0;

        let (id, item) = item.unwrap();
        for items in item.symbols.clone() {
           for item in items {
               if item.is_none() { fskip += 1; continue }
               let item = item.unwrap();

               let mut c = item.chars();

               let fchar = c.next().unwrap();
               let lchar = c.next().unwrap();

               let final_item = if lchar.eq(&'w') {
                   fchar.to_ascii_uppercase()
               } else { fchar.to_ascii_lowercase() };

               if fskip > 0 {
                   fen.push_str(&format!("{fskip}"));
                   fskip = 0;
               }

               fen.push(final_item);
           }

            fen.push('/');
        }
        fen.remove(fen.len() - 1);

        println!("Fen for {} ({}): {}", v, id, fen);
    }
}
