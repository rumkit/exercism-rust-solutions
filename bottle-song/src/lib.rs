pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut output = String::new();
    for x in 0..take_down {
        output.push_str(&verse(start_bottles - x));
        output.push('\n');
    }

    output
}
fn verse(num: u32) -> String {
    format!(
        "{0} green {1} hanging on the wall,\n\
         {0} green {1} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {2} green {3} hanging on the wall.\n",
        stringify_number(num),
        bottles(num),
        stringify_number(num - 1).to_lowercase(),
        bottles(num - 1)
    )
}

fn bottles(n: u32) -> &'static str {
    match n {
        1 => "bottle",
        _ => "bottles",
    }
}

fn stringify_number(n: u32) -> &'static str {
    match n {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => panic!("can't process the number"),
    }
}
