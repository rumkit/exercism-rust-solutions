use std::fs;
use anyhow::Error;

#[derive(Debug, Default)]
pub struct Flags {
    prepend_line_number: bool,
    filenames_only: bool,
    case_insensitive: bool,
    invert_search: bool,
    match_entire_line_only: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        flags.iter().fold(Flags::default(), |mut flags, &x| {
            match x.trim() {
                "-n" => flags.prepend_line_number = true,
                "-l" => flags.filenames_only = true,
                "-i" => flags.case_insensitive = true,
                "-v" => flags.invert_search = true,
                "-x" => flags.match_entire_line_only = true,
                _ => panic!("Unknown flag: {}", x),
            }
            flags
        })
    }
}

type MatcherFn = Box<dyn Fn(&str, &str) -> bool>;
struct Matcher {
    pattern: String,
    matcher_fn: MatcherFn,
}

impl Matcher {
    pub fn new(pattern: &str, flags: &Flags) -> Self {
        let mut base_fn: MatcherFn = match flags.match_entire_line_only {
            false => Box::new(|pattern: &str, input: &str| input.contains(pattern)),
            true => Box::new(|pattern, input: &str| input == pattern),
        };
        if flags.case_insensitive {
            base_fn = Box::new(move |pattern: &str, input: &str| base_fn(&pattern.to_ascii_lowercase(), &input.to_ascii_lowercase()));
        }

        if flags.invert_search {
            base_fn = Box::new(move |pattern: &str, input: &str| !base_fn(pattern, input));
        }

        Self { pattern: pattern.to_string(), matcher_fn: base_fn}
    }

    pub fn matches(&self, input: &str) -> bool {
        (self.matcher_fn)(&self.pattern, input)
    }
}


pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let prepend_filename = files.len() > 1;
    files.iter()
        .map(|file| grep_file(pattern, flags, file, prepend_filename))
        .try_fold(Vec::new(), |mut acc, result| {
            acc.extend(result?);
            Ok(acc)
        })
}

fn grep_file(pattern: &str, flags: &Flags, file: &str, prepend_filename: bool) -> Result<Vec<String>, Error> {
    let contents = fs::read_to_string(file)?;
    let mut result =  vec![];
    let mut append_result =  |s: String|
        if prepend_filename {result.push(format!("{}:{}",file, s))} else {result.push(s)};
    let matcher = Matcher::new(pattern, flags);

    for (i, line) in contents.lines().enumerate() {
        if matcher.matches(line) {
            match flags {
                Flags {filenames_only: true, invert_search: false, ..} => {
                    return Ok(vec![file.to_string()])
                }
                Flags { prepend_line_number: true, ..} => {
                    append_result(format!("{}:{}", i + 1, line));
                }
                _ => {
                    append_result(line.to_string());
                }
            }
        } else {
            // return fast if any line matches during full file invert search
            if let Flags {filenames_only: true, invert_search: true, ..} = flags {
                return Ok(vec![])
            }
        }
    }

    if flags.invert_search && flags.filenames_only {
        return Ok(vec![file.to_string()])
    }

    Ok(result)
}
