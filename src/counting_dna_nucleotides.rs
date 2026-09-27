use std::path::Path;
use crate::read::read_all;

pub fn count () {
    let sequence = read_all (Path::new("dataset/rosalind_dna.txt"));
    let mut nucleotides = (0, 0, 0, 0);

    for nucleotide in sequence.chars() {
        match nucleotide {
            'A' => nucleotides.0 += 1,
            'C' => nucleotides.1 += 1,
            'G' => nucleotides.2 += 1,
            'T' => nucleotides.3 += 1,
            _ => continue,
        }
    }
    println!("{} {} {} {}", nucleotides.0, nucleotides.1, nucleotides.2, nucleotides.3);
}