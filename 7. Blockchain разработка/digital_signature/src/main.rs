use ed25519_dalek::ed25519::signature::Signer;
use ed25519_dalek::{Signature, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

fn verify(msg: &[u8], sig: &[u8], key: &[u8]) -> bool {
    if key.len() != 32 || sig.len() != 64 {
        return false;
    }

    let key = key.try_into().unwrap();
    let sig = sig.try_into().unwrap();
    let verifying_key = VerifyingKey::from_bytes(key).unwrap();
    let signature = Signature::from_bytes(sig);
    verifying_key.verify(msg, &signature).is_ok()
}

fn main() {
    let msg = "Send 5 SOL to Alice";

    let mut rng = OsRng;
    let signing_key1 = SigningKey::generate(&mut rng);
    let signing_key2 = SigningKey::generate(&mut rng);

    println!("Сообщение: {}", msg);
    println!("Приватный ключ 1: {}", hex::encode(signing_key1.to_bytes()));
    println!(
        "Публичный ключ 1: {}",
        hex::encode(signing_key1.verifying_key().to_bytes())
    );
    println!("Приватный ключ 2: {}", hex::encode(signing_key2.to_bytes()));
    println!(
        "Публичный ключ 2: {}",
        hex::encode(signing_key2.verifying_key().to_bytes())
    );

    assert!(verify(
        msg.as_bytes(),
        &signing_key1.sign(msg.as_bytes()).to_bytes(),
        signing_key1.verifying_key().as_bytes()
    ));

    let new_msg = "Send 10 SOL to Alice";
    println!("Заменяем сообщение: {new_msg}");

    assert!(!verify(
        new_msg.as_bytes(),
        &signing_key1.sign(msg.as_bytes()).to_bytes(),
        signing_key1.verifying_key().as_bytes()
    ));

    println!("Пытаемся заменить цифровую подпись");
    assert!(!verify(
        msg.as_bytes(),
        &signing_key2.sign(msg.as_bytes()).to_bytes(),
        signing_key1.verifying_key().as_bytes()
    ));
}
