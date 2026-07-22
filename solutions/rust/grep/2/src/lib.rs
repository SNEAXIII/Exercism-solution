use anyhow::{Context, Error};
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let file = File::open(file_path).context("File does not exist!")?;
        let reader = BufReader::new(file);
        for (line, index) in reader.lines().zip(1..) {
            let line = line.context("Error while reading the file")?;
            let line_to_search: Cow<str> = if flags.case_insensitive {
                Cow::Owned(line.to_lowercase())
            } else {
                Cow::Borrowed(&*line)
            };
            if ((flags.full_match && pattern != line_to_search)
                || !line_to_search.contains(&pattern))
                ^ flags.reverse_condition
            {
                continue;
            }
            if flags.add_file_names {
                results.push(file_path.to_string());
                break;
            }
            let mut output = String::new();
            if is_multiple_files {
                output.push_str(*file_path);
                output.push(':');
            }
            if flags.add_number {
                output.push_str(&index.to_string());
                output.push(':');
            }
            if output.is_empty() {
                results.push(line);
            } else {
                output.push_str(&line);
                results.push(output);
            }
        }
    }

    Ok(results)
}
