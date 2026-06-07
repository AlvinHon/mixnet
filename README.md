 # mixnet

Mixnet is a communication architecture that provides anonymous messaging by shuffling messages and wrapping them in layers of encryption. This work is rust implementation of the paper [Efficient Verifiable Mixnets from Lattices, Revisited"](https://eprint.iacr.org/2025/658.pdf).


**Components**

- Lattice-based HPKE primitives (`hpke`) compatible with MLWE instantiations
- OTSE (one time symmetric encryption) (`otse`) for message encoding
- Ajtai-based commitments and openings (`ajtai`)

## Protocol specification

1. Each mixnet layer holds an HPKE key pair. Messages are encoded into polynomial vectors and encrypted using the layer's public key.
2. A ciphertext can be wrapped to target the next layer using `encrypt_next`, producing a nested ciphertext chain.
3. A layer receives a batch of ciphertexts, **shuffles** the batch, **decrypts** each ciphertext, and forwards either the next-layer ciphertexts or the plaintext messages.

## Code example

Minimal example showing two-layer encryption, shuffle, and decryption. This follows the crate's tests and demonstrates the typical flow.

```rust
let rng = &mut rand::rng();
const L: usize = 2; // length of message vector

let otse_params = mixnet::otse::create_default_params::<L, _>(rng);

let mixnet_layer_1 = {
    let (hpke_pk, hpke_sk) = mixnet::hpke::keygen(otse_params.clone(), rng);
    mixnet::MixnetLayer::new(hpke_pk, hpke_sk)
};
let mixnet_layer_2 = {
    let (hpke_pk, hpke_sk) = mixnet::hpke::keygen(otse_params, rng);
    mixnet::MixnetLayer::new(hpke_pk, hpke_sk)
};

// define messages
let m1 = mixnet::hpke::HpkeMessage::try_from([vec![1i64, 2, 3, 4], vec![5i64, 6, 7, 8]]).unwrap();
let m2 = mixnet::hpke::HpkeMessage::try_from([vec![-9i64, 10, 11, 12], vec![-13i64, 14, 15, 16]])
    .unwrap();

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

// shuffle and decrypt in second Layer
let shuffle_result = mixnet_layer_2.shuffle(c2, rng);
let mixnet::ShuffleResult::DecryptedWithNextCiphertexts(next_ciphertexts) = shuffle_result else {
    panic!("Expected DecryptedWithNextCiphertexts");
};

// shuffle and decrypt in first layer
let shuffle_result = mixnet_layer_1.shuffle(next_ciphertexts, rng);
let mixnet::ShuffleResult::Decrypted(decrypted_messages) = shuffle_result else {
    panic!("Expected Decrypted");
};

assert_eq!(decrypted_messages.len(), 2);
// either decrypted_messages[0] corresponds to m[0] or m[1] since the order is shuffled
assert!(
    (decrypted_messages[0] == m1 && decrypted_messages[1] == m2)
        || (decrypted_messages[0] == m2 && decrypted_messages[1] == m1)
);
```

## Contributing

Contributions are welcome. Please open issues or pull requests.

This work is not fully complete - the proof of shuffle stated in the paper is not yet included in this crate. The difficulty is that the proof of shuffle depends on a ZK proof system which is not concretely defined. Highly appreciated if contributers could implement it!