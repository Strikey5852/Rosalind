use std::path::Path;
use crate::read::read_all;

pub fn transcribe () {
    let dna = read_all(Path::new("dataset/rosalind_rna.txt"));
    let rna = dna.replace("T", "U");

    println!("{}", rna);
}