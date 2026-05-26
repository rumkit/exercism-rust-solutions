const SUFFIXES: &[&str] = &[
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion"
];

pub fn encode(mut n: u64) -> String {
    let mut suffix_n = 0;
    let mut result = String::new();
    loop {
        if n % 1000 > 0 || n < 1000 {
            result = format!("{encoded} {suffix} {result}", encoded = encode_int(n % 1000), suffix = SUFFIXES[suffix_n]);
        }
        n /= 1000;
        suffix_n += 1;
        if n == 0 {
            break;
        }
    }

    result.trim_end().to_string()
}

fn encode_int(n: u64) -> String {
    if n == 0 {
        return encode_units(n);
    }

    let hundreds = n / 100;
    let mut result = match hundreds {
        0 => String::new(),
        hundreds => format!("{hundreds} hundred", hundreds = encode_units(hundreds)),
    };

    match n % 100 {
        0 => {},
        remainder => {
            if hundreds > 0 { result += " " };
            match remainder {
                remainder if (10..=19).contains(&remainder) => {
                    result += &encode_units(remainder);
                }
                remainder=> {
                    let units = remainder % 10;
                    result += &encode_tens(remainder - units);
                    if remainder >= 20 && units > 0 {
                        result += "-";
                    }
                    if units > 0 {
                        result+= &encode_units(units);
                    }
                }
            }
        }
    }
    result
}

fn encode_units(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        _ => panic!("invalid units")
    }
}

fn encode_tens(n: u64) -> String {
    match n {
        0 => String::from(""),
        10 => String::from("ten"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        _ => panic!("invalid tens")
    }
}
