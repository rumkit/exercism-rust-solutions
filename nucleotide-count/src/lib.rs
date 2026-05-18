use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna).and_then(|map| map.get(&nucleotide).copied().ok_or(nucleotide))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hashmap = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {hashmap.entry(c).and_modify(|e| *e += 1); },
            _ => return Err(c)
        }
    }

    Ok(hashmap)
}
