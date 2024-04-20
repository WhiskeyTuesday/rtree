use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use ansi_term::Colour::{Cyan, White};
use ansi_term::Style;

fn line_count<P: AsRef<Path>>(file_path: P) -> usize {
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().count()
}

fn walk_tree<P: AsRef<Path>>(dir_path: P, prefix: String, color: Style, sort_by_lines: bool) {
    let entries = fs::read_dir(&dir_path).expect("Failed to read directory");
    let mut entries_vec: Vec<_> = entries.map(|res| res.expect("Failed to read entry")).collect();

    if sort_by_lines {
        entries_vec.sort_by(|a, b| {
            line_count(&a.path()).cmp(&line_count(&b.path()))
        });
    } else {
        entries_vec.sort_by(|a, b| {
            a.file_name().to_string_lossy().to_lowercase().cmp(&b.file_name().to_string_lossy().to_lowercase())
        });
    }

    let len = entries_vec.len();
    for (index, entry) in entries_vec.iter().enumerate() {
        let last = index == len - 1;
        let entry_path = entry.path();
        let connector = if last { "└── " } else { "├── " };
        let next_prefix = if last { "    " } else { "│   " };

        let formatted_name = if entry_path.is_dir() {
            Cyan.paint(entry.file_name().to_string_lossy().to_string() + "/").to_string()
        } else {
            let count = line_count(&entry_path);
            format!("{}{}, {} lines", White.paint(entry.file_name().to_string_lossy().to_string()), color.paint(""), count)
        };

        println!("{}{}{}", prefix, connector, formatted_name);

        if entry_path.is_dir() {
            walk_tree(entry_path, format!("{}{}", prefix, next_prefix), color, sort_by_lines);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (dir_path, sort_by_lines) = if args.len() > 1 {
        if args[1] == "-l" {
            if args.len() > 2 {
                (args[2].as_str(), true)
            } else {
                (".", true)
            }
        } else {
            (args[1].as_str(), false)
        }
    } else {
        (".", false)
    };

    let color = White.underline();
    println!("Full path: {}", fs::canonicalize(dir_path).unwrap().display());
    walk_tree(Path::new(dir_path), String::from(""), color, sort_by_lines);
}
