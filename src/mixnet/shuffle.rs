#[derive(Debug, Clone)]
pub struct Ciphertext(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct ShuffleProof {
    pub bytes: Vec<u8>,
}

pub fn reencrypt_and_shuffle(input: &[Ciphertext], permutation: &[usize]) -> Vec<Ciphertext> {
    let mut out = Vec::with_capacity(input.len());
    for &idx in permutation {
        out.push(input[idx].clone());
    }
    out
}
