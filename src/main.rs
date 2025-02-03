use clap::Parser;
use csv::Writer;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    path::{absolute, PathBuf},
    process::exit,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    folder: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn write_to_csv(output: PathBuf, reproducers: HashMap<String, String>) {
    let csv_path = absolute(output.join("output.csv")).unwrap();

    let mut wtr = Writer::from_path(csv_path);

    for (repro_path, repro_type) in &reproducers {
        let _ = wtr
            .as_mut()
            .expect("Could not write value to csv")
            .write_record([repro_path, repro_type]);
    }
    let _ = wtr.expect("Could not flush csv").flush();
}

fn main() {
    let mut args = Args::parse();

    let unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    args.output = absolute(args.output.join(unixtime.to_string()))
        .expect("Could not get absolute path for args.output");
    let folder_absolute = absolute(args.folder).expect("Could not get absolute path");

    if !folder_absolute.exists() {
        println!("Provided folder doesn't exist. Please enter a valid folder name");
        exit(1);
    }
    if !args.output.exists() {
        let res = create_dir_all(&args.output);
        match res {
            Ok(_) => (),
            Err(e) => println!("Error occurred: {}", e),
        }
    }

    // Hashmap to store reproducer paths and their corresponding types
    let mut reproducers: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(&folder_absolute)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry = entry.into_path();
        if entry.is_file() {
            if entry.to_string_lossy().contains("cprog") {
                reproducers.insert(entry.to_string_lossy().to_string(), "C".to_string());
            } else if entry.to_string_lossy().contains("repro0")
                || entry.to_string_lossy().contains(".prog")
            {
                reproducers.insert(entry.to_string_lossy().to_string(), "Syz".to_string());
            }
        }
    }

    write_to_csv(args.output.clone(), reproducers);
}
