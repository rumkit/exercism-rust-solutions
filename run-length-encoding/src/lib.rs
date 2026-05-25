pub fn encode(source: &str) -> String {
    let mut last_char: char = '0';
    let mut counter = 0;
    let mut output = String::new();

    for c in source.chars() {
        if c == last_char {
            counter += 1;
        }
        else {
            append_output(&mut output, counter, last_char);
            last_char = c;
            counter = 1;
        }
    }
    append_output(&mut output, counter, last_char);

    output
}

fn append_output(output: &mut String, counter: u32, c: char) {
    if c == '0' {
        return;
    }
    if counter > 1 {
        output.push_str(&counter.to_string());
    }
    output.push(c);
}

pub fn decode(source: &str) -> String {
    let mut output = String::new();
    let mut counter: usize = 0;
    for c in source.chars() {
        if let Some(d) = c.to_digit(10) {
            counter = counter * 10 + d as usize;
        } else {
            output.extend(std::iter::repeat_n(c, if counter == 0 {1} else {counter}));
            counter = 0;
        }
    }

    output
}
