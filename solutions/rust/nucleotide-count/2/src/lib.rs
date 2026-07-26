use std::collections::HashMap;

fn is_nucleotide(nucleotide: char) -> bool {
    matches!(nucleotide, 'A' | 'C' | 'G' | 'T')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }
    let mut result: usize = 0;
    for n in dna.chars() {
        if n == nucleotide {
            result += 1;
        } else if !is_nucleotide(n) {
            return Err(n);
        }
    }
    Ok(result)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let (mut a, mut c, mut g, mut t) = (0, 0, 0, 0);
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' => a += 1,
            'C' => c += 1,
            'G' => g += 1,
            'T' => t += 1,
            _ => return Err(nucleotide),
        }
    }
    Ok(HashMap::from([('A', a), ('C', c), ('G', g), ('T', t)]))
}
