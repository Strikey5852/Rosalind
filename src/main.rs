
mod read;
mod counting_dna_nucleotides;
mod transcribing_dna_into_rna;

fn main() {
    counting_dna_nucleotides::count();
    transcribing_dna_into_rna::transcribe();
}