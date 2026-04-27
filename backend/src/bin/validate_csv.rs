use std::{env, fs::File, path::Path};

use csv::ReaderBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: validate_csv <path-to-csv>");
        std::process::exit(2);
    }

    let path = &args[1];
    if !Path::new(path).exists() {
        eprintln!("File not found: {}", path);
        std::process::exit(2);
    }

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            std::process::exit(2);
        }
    };

    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut total = 0usize;
    let mut ok = 0usize;
    let mut bad = 0usize;
    let mut details: Vec<(usize, String)> = Vec::new();

    for (i, result) in rdr.records().enumerate() {
        match result {
            Ok(rec) => {
                total += 1;
                let org_id = rec.get(0).unwrap_or("").trim();
                let mut accepted = false;
                if !org_id.is_empty() {
                    let up = org_id.to_uppercase();
                    let low = org_id.to_lowercase();
                    if up.starts_with("ORG")
                        || up.starts_with("IR")
                        || up.starts_with("IT")
                        || low.starts_with("org_id")
                    {
                        accepted = true;
                    }
                }
                if accepted {
                    ok += 1;
                } else {
                    bad += 1;
                    // row number considering header + first data row = 2
                    details.push((i + 2, org_id.to_string()));
                }
            }
            Err(e) => {
                eprintln!("CSV parse error at record {}: {}", i + 2, e);
            }
        }
    }

    println!("Total rows: {}", total);
    println!("Accepted rows: {}", ok);
    println!("Rejected rows: {}", bad);
    if !details.is_empty() {
        println!("Rejected details (row, org_id):");
        for (r, v) in details.iter() {
            println!("{}, {}", r, v);
        }
    }
}
