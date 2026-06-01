pub fn answer(command: &str) -> Option<i32> {
    let mut pending_action: Option<fn(i32, i32) -> i32> = None;
    let result = into_subcommands(command, |sc, state| {
        match sc {
            Subcommand::Number(_) if pending_action.is_none() => None,
            Subcommand::Number(n) => {
                let action = pending_action.take().unwrap();
                Some(action(state, n))
            },
            Subcommand::Action(action) => {
                pending_action = parse_action(action);
                pending_action.map(|_| state)
            }
        }
    })?;

    Some(result)
}

fn parse_action(action: &str) -> Option<fn(i32, i32) -> i32> {
    match action {
        "What is" => Some(|state, number| state + number),
        "plus" => Some(|state, number| state + number),
        "minus" => Some(|state, number| state - number),
        "divided by" => Some(|state, number| state / number),
        "multiplied by" => Some(|state, number| state * number),
        _ => None
    }
}

enum Subcommand<'a> {
    Action(&'a str),
    Number(i32),
}

fn into_subcommands<F: FnMut(Subcommand, i32) -> Option<i32>>(command: &str, mut handler: F) -> Option<i32> {
    let mut start = 0;
    let mut is_previous_digit = false;
    let mut state = 0;
    for (i, c) in command.char_indices() {
        if (c.is_ascii_digit() || c == '-') != is_previous_digit {
            // if digits changes to non-digit or vice versa ('-' considered as digit here)
            let substr = command[start..i].trim();
            state = handler( match is_previous_digit {
                true => { Subcommand::Number(substr.parse().unwrap())},
                false => { Subcommand::Action(substr) }
            }, state)?;
            start = i;
            is_previous_digit = c.is_ascii_digit() || c == '-';
        } else if i == command.len() - 1 {
            // last token (anything after the last digit)
            let substr = command[start..=i].trim();
            state = handler( match is_previous_digit {
                true => { Subcommand::Number(substr.parse().unwrap())},
                false => { Subcommand::Action(substr) }
            }, state)?;
        }
    }

    Some(state)
}
