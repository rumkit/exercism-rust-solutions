const ACTIONS: &[(u8, &str)] = &[
    (0b00001, "wink"),
    (0b00010, "double blink"),
    (0b00100, "close your eyes"),
    (0b01000, "jump")
];

pub fn actions(n: u8) -> Vec<&'static str> {
    let reverse = (n & (1 << 4) as u8) > 0;
    let actions = ACTIONS.iter()
        .filter_map(|&(mask, action)| (n & mask > 0).then_some(action));

    if reverse {
        actions.rev().collect()
    }
    else {
        actions.collect()
    }
}
