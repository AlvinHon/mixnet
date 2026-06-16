use serde_derive::{Deserialize, Serialize};

use crate::{mlwe::MlweCiphertext, otse::OTSEEncoded};

/// HPKE ciphertext type
///
/// Size (i.e. number of elements) depends on number of layer of encryption:
/// Size = KR * (size of MLWE ciphertext) + size of OTSE ciphertext
///      = KR * (KE + 1) + L
///
/// For subsequent layers of encryption, the size will be added by KR * (KE + 1) for each layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HpkeCiphertext<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const KR: usize,
    const L: usize,
> {
    pub(crate) c: Vec<MlweCiphertext<Q, N, KE>>, // KR ciphertexts for first layer encryption
    pub(crate) cs: OTSEEncoded<Q, N, L>,
}
