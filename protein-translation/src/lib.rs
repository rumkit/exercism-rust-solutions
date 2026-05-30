use std::mem::replace;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    rna.char_indices()
        .scan((0, 0), |(count, start), (i, _)| {
            *count += 1;
            if *count == 3 {
                // return chunk of 3
                *count = 0;
                let s = replace(start, i + 1);
                Some(&rna[s..=i])
            } else if i == rna.len() - 1 {
                // return the remainder of str if len() % 3 != 0
                Some(&rna[*start..=i])
            } else {
                // return a stub that we'll filter out
                Some("")
            }
        })
        .filter(|codon| !codon.is_empty())
        .take_while(|codon| !["UAA", "UAG", "UGA"].contains(codon))
        .map(|codon| match codon {
            "AUG" => Some("Methionine"),
            "UUU" | "UUC" => Some("Phenylalanine"),
            "UUA" | "UUG" => Some("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
            "UAU" | "UAC" => Some("Tyrosine"),
            "UGU" | "UGC" => Some("Cysteine"),
            "UGG" => Some("Tryptophan"),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
}
