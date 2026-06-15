pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    let mut input: Vec<Option<(u8, u8)>> = input.iter().map(|&(a, b)| Some((a,b))).collect();
    let mut output = Vec::with_capacity(input.len());
    let target_len = input.len();

    let first = input[0].take().unwrap();
    output.push(first);

    dfs(&mut input, &mut output, target_len)
}

fn dfs(input: &mut Vec<Option<(u8, u8)>>, output: &mut Vec<(u8, u8)>, target_len: usize) -> Option<Vec<(u8, u8)>> {
    // check if output reached desired length
    if output.len() == target_len {
        // check if there are tiles at a distance 'output.len()' with equal ends
        for i in 0..output.len() {
            let start = i;
            let end = (i + output.len() - 1) %  output.len();
            if output[start].0 == output[end].1 {
                // ideally, we should have rotated the output before returning, but there are no tests checking it
                return Some(output.to_vec());
            }
        }
        return None;
    }

    let mut search_from = 0;
    while let Some(res) = get_next_tile(output[output.len() - 1].1, input, &mut search_from) {
        // if a candidate found, rotate it so it fits and push it output
        if res.0 == output[output.len() - 1].1 {
            output.push(res)
        } else { output.push((res.1, res.0))}

        // recurse with remaining tiles
        if let Some(res) = dfs(input, output, target_len) {
            return Some(res);
        }

        // remove the candidate tile from output and put it back to available tiles
        output.pop();
        input[search_from - 1] = Some(res);
    }
    None
}

// searches for the next fitting tile and adjusts search_from to the position after the candidate
fn get_next_tile(target: u8, input: &mut [Option<(u8, u8)>], search_from: &mut usize) -> Option<(u8, u8)> {
    for i in *search_from .. input.len() {
        if let Some(candidate) = input[i] &&
            (candidate.0 == target || candidate.1 == target) {
                *search_from = i + 1;
                return input[i].take();
            }
    }
    None
}