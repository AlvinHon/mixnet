// Copyright 2026 Alvin Hon. See the COPYRIGHT file LICENSE-XXX at the root folder.

#![doc = include_str!("../README.md")]

pub mod ajtai;
pub mod hpke;
pub mod mlwe;
pub mod otse;
pub mod preliminaries;

use crate::hpke::{
    HpkeCiphertext, HpkeMessage, HpkePublicKey, HpkeSecretKey, decrypt::HpkeDecryptResult,
};

pub struct MixnetLayer<
    const Q: i64,
    const N: usize,
    const KE: usize, // K_lwe
    const Z: i64,    // 2^z
    const E: i64,    // 2^(2*eta)
    const KR: usize, // K_lwr
    const L: usize,
> {
    pub(crate) hpke_pk: HpkePublicKey<Q, N, KE, Z, E, KR, L>,
    pub(crate) hpke_sk: HpkeSecretKey<Q, N, KE, Z, E, KR, L>,
}

impl<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const Z: i64,
    const E: i64,
    const KR: usize,
    const L: usize,
> MixnetLayer<Q, N, KE, Z, E, KR, L>
{
    pub fn new(
        hpke_pk: HpkePublicKey<Q, N, KE, Z, E, KR, L>,
        hpke_sk: HpkeSecretKey<Q, N, KE, Z, E, KR, L>,
    ) -> Self {
        Self { hpke_pk, hpke_sk }
    }

    pub fn public_key(&self) -> HpkePublicKey<Q, N, KE, Z, E, KR, L> {
        self.hpke_pk.clone()
    }

    pub fn shuffle<R: rand::Rng + ?Sized>(
        &self,
        ciphertexts: Vec<HpkeCiphertext<Q, N, KE, KR, L>>,
        rng: &mut R,
    ) -> ShuffleResult<Q, N, KE, KR, L> {
        use rand::seq::SliceRandom;
        if ciphertexts.is_empty() {
            return ShuffleResult::Failure("No ciphertexts provided".to_string());
        }

        let mut shuffled_ciphertexts = ciphertexts;
        shuffled_ciphertexts.shuffle(rng); // TODO should shuffle by function g

        let decrypted_result = shuffled_ciphertexts
            .into_iter()
            .map(|c| self.hpke_sk.decrypt(&c))
            .collect::<Vec<_>>();

        let mut decrypted_ciphertexts = Vec::new();
        let mut decrypted_messages = Vec::new();

        for result in decrypted_result {
            match result {
                HpkeDecryptResult::DecryptedWithNextCiphertext(next_ciphertexts) => {
                    decrypted_ciphertexts.push(next_ciphertexts);
                }
                HpkeDecryptResult::DecryptedMessage(m) => {
                    decrypted_messages.push(m);
                }
                HpkeDecryptResult::DecryptionFailed(err) => {
                    return ShuffleResult::Failure(format!("Decryption failed: {}", err));
                }
            }
        }

        if !decrypted_ciphertexts.is_empty() {
            ShuffleResult::DecryptedWithNextCiphertexts(decrypted_ciphertexts)
        } else {
            ShuffleResult::Decrypted(
                decrypted_messages
                    .into_iter()
                    .map(HpkeMessage::from)
                    .collect(),
            )
        }
    }
}

pub enum ShuffleResult<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const KR: usize,
    const L: usize,
> {
    DecryptedWithNextCiphertexts(Vec<HpkeCiphertext<Q, N, KE, KR, L>>),
    Decrypted(Vec<HpkeMessage<Q, N, L>>),
    Failure(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mixnet_layer() {
        let rng = &mut rand::rng();
        const L: usize = 2; // length of message vector

        let otse_params = otse::create_default_params::<L, _>(rng);

        let mixnet_layer_1 = {
            let (hpke_pk, hpke_sk) = hpke::keygen(otse_params.clone(), rng);
            MixnetLayer::new(hpke_pk, hpke_sk)
        };
        let mixnet_layer_2 = {
            let (hpke_pk, hpke_sk) = hpke::keygen(otse_params, rng);
            MixnetLayer::new(hpke_pk, hpke_sk)
        };

        let m1 = hpke::HpkeMessage::random(rng);
        let m2 = hpke::HpkeMessage::random(rng);

        // first layer encryption
        let c1 = vec![
            mixnet_layer_1.public_key().encrypt(&m1, rng),
            mixnet_layer_1.public_key().encrypt(&m2, rng),
        ];

        // second layer encryption
        let c2 = vec![
            mixnet_layer_2.public_key().encrypt_next(&c1[0], rng),
            mixnet_layer_2.public_key().encrypt_next(&c1[1], rng),
        ];

        let shuffle_result = mixnet_layer_2.shuffle(c2, rng);
        let next_ciphertexts =
            if let ShuffleResult::DecryptedWithNextCiphertexts(next_ciphertexts) = shuffle_result {
                next_ciphertexts
            } else {
                panic!("Expected DecryptedWithNextCiphertexts");
            };
        let shuffle_result = mixnet_layer_1.shuffle(next_ciphertexts, rng);
        if let ShuffleResult::Decrypted(decrypted_messages) = shuffle_result {
            assert_eq!(decrypted_messages.len(), 2);
            // either decrypted_messages[0] corresponds to m[0] or m[1] since the order is shuffled
            assert!(
                (decrypted_messages[0] == m1 && decrypted_messages[1] == m2)
                    || (decrypted_messages[0] == m2 && decrypted_messages[1] == m1)
            );
        } else {
            panic!("Expected Decrypted");
        }
    }
}
