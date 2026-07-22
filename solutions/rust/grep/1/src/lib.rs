use anyhow::Context;
use anyhow::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct Flags {
    reverse_condition: bool,
    full_match: bool,
    add_number: bool,
    add_file_names: bool,
    case_insensitive: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            reverse_condition: (flags.contains(&"-v")),
            full_match: (flags.contains(&"-x")),
            add_number: (flags.contains(&"-n")),
            add_file_names: (flags.contains(&"-l")),
            case_insensitive: (flags.contains(&"-i")),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    if files.is_empty() {
        return Err(anyhow::anyhow!("No files provided!"));
    }
    let pattern: String = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let is_multiple_files = files.len() > 1;
    let mut results: Vec<String> = Vec::new();
    for file_path in files {
        fs::metadata(file_path).context("File does not exist!")?;
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for (line, index) in reader.lines().zip(1..) {
            let mut line = match line {
                Ok(line) => line,
                Err(_) => return Err(anyhow::anyhow!("Error while reading the file")),
            };
            let line_to_search: String = if flags.case_insensitive {
                line.to_lowercase()
            } else {
                line.clone()
            };
            if ((flags.full_match && !(pattern == line_to_search))
                || !line_to_search.contains(&pattern))
                ^ flags.reverse_condition
            {
                continue;
            }

            if flags.add_number {
                line.insert(0, ':');
                line.insert_str(0, &index.to_string());
            }
            if is_multiple_files {
                line.insert(0, ':');
                line.insert_str(0, *file_path);
            }
            if flags.add_file_names {
                let to_add = file_path.to_string();
                if !results.contains(&to_add) {
                    results.push(to_add);
                }
            } else {
                results.push(line);
            }
        }
    }

    Result::Ok(results)
}
