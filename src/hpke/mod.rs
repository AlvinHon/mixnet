pub mod encrypt;
pub use encrypt::HpkePublicKey;
pub mod decrypt;
pub use decrypt::HpkeSecretKey;
pub mod ciphertext;
pub use ciphertext::HpkeCiphertext;
pub mod message;
pub use message::HpkeMessage;

pub fn keygen<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const Z: i64,
    const E: i64,
    const KR: usize,
    const L: usize,
    R: rand::Rng + ?Sized,
>(
    otse_params: crate::otse::OTSEParams<Q, N, KE, Z, E, KR, L>,
    rng: &mut R,
) -> (
    HpkePublicKey<Q, N, KE, Z, E, KR, L>,
    HpkeSecretKey<Q, N, KE, Z, E, KR, L>,
) {
    let (pk, sk) = crate::mlwe::keygen(otse_params.eta, rng);
    (
        HpkePublicKey {
            otse_params: otse_params.clone(),
            pk,
        },
        HpkeSecretKey { otse_params, sk },
    )
}

#[cfg(test)]
mod test {
    use crate::hpke::decrypt::HpkeDecryptResult;

    use super::*;

    #[test]
    fn test_hpke_keygen() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const KE: usize = 2;
        const Z: i64 = 64; // 2^6
        const E: i64 = 16; // 2^(2*2)
        const KR: usize = 2;
        const L: usize = 2;

        let rng = &mut rand::rng();

        let otse_params = crate::otse::OTSEParams::<Q, N, KE, Z, E, KR, L>::new(2, rng);
        let (pk, sk) = keygen(otse_params, rng);
        assert_eq!(pk.otse_params.eta, 2);
        assert_eq!(pk.otse_params.h.len(), KE + L);
        assert_eq!(pk.otse_params.h[0].len(), KR);
        assert_eq!(pk.otse_params.h1.len(), L);
        assert_eq!(pk.otse_params.h1[0].len(), KE);
        assert_eq!(sk.otse_params.eta, 2);
        assert_eq!(sk.otse_params.h.len(), KE + L);
        assert_eq!(sk.otse_params.h[0].len(), KR);
        assert_eq!(sk.otse_params.h1.len(), L);
        assert_eq!(sk.otse_params.h1[0].len(), KE);
    }

    #[test]
    fn test_hpke_encrypt_decrypt() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const KE: usize = 2;
        const Z: i64 = 64; // 2^6
        const E: i64 = 16; // 2^(2*2)
        const KR: usize = 2;
        const L: usize = 2;

        let rng = &mut rand::rng();

        let otse_params = crate::otse::OTSEParams::<Q, N, KE, Z, E, KR, L>::new(2, rng);
        let (pk, sk) = keygen(otse_params, rng);

        let m = HpkeMessage::<Q, N, L>::random(rng);
        let ciphertext = pk.encrypt(&m, rng);
        let decrypt_result = sk.decrypt(&ciphertext);
        match decrypt_result {
            HpkeDecryptResult::DecryptedMessage(decrypted_m) => {
                assert_eq!(decrypted_m[0], m.m[0]);
                assert_eq!(decrypted_m[1], m.m[1]);
            }
            _ => panic!("Decryption failed"),
        }
    }

    #[test]
    fn test_hpke_decrypt_with_next_ciphertext() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const KE: usize = 2;
        const Z: i64 = 64; // 2^6
        const E: i64 = 16; // 2^(2*2)
        const KR: usize = 2;
        const L: usize = 2;

        let rng = &mut rand::rng();

        let otse_params = crate::otse::OTSEParams::<Q, N, KE, Z, E, KR, L>::new(2, rng);
        let (pk1, sk2) = keygen(otse_params.clone(), rng);
        let (pk2, sk1) = keygen(otse_params.clone(), rng);

        let m = HpkeMessage::<Q, N, L>::random(rng);
        let ciphertext1 = pk1.encrypt(&m, rng);
        let ciphertext2 = pk2.encrypt_next(&ciphertext1, rng);

        let decrypt_result1 = sk1.decrypt(&ciphertext2);
        let decrypted_1 = match decrypt_result1 {
            HpkeDecryptResult::DecryptedWithNextCiphertext(next_ciphertext) => next_ciphertext,
            _ => panic!("Decryption failed at first layer"),
        };

        let decrypt_result2 = sk2.decrypt(&decrypted_1);
        match decrypt_result2 {
            HpkeDecryptResult::DecryptedMessage(decrypted_m) => {
                assert_eq!(decrypted_m[0], m.m[0]);
                assert_eq!(decrypted_m[1], m.m[1]);
            }
            _ => panic!("Decryption failed at second layer"),
        }
    }
}
