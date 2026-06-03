pub fn get_diamond(c: char) -> Vec<String> {
    let size = ((c as u8 - b'A') * 2 + 1)  as usize;
    let bottom = ('A'..=c).rev()
        .enumerate()
        .map(|(i, c)| {
            let x = match c {
                'A' => "A".to_string(),
                _ => {
                    let spacer = " ".repeat(size - 2 - i*2);
                    format!("{c}{spacer}{c}")
                }
            };
            format!("{:^width$}", x, width = size)
        })
        .collect::<Vec<_>>();

    let mut top = bottom.iter().cloned().rev().take(size/2).collect::<Vec<_>>();
    top.extend(bottom);
    top
}
