use oqs::kem;

fn main() {
    println!("VEILNET — Quantum Safe Network");
    println!("GhostNode1 online.");
    println!("");
    
    // Schlüssel generieren
    let kem = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
    let (public_key, secret_key) = kem.keypair().unwrap();
    println!("Quantum-safe keys generated!");
    
    // Nachricht verschlüsseln
    let (ciphertext, shared_secret_sender) = kem.encapsulate(&public_key).unwrap();
    println!("Message encrypted!");
    
    // Nachricht entschlüsseln
    let shared_secret_receiver = kem.decapsulate(&secret_key, &ciphertext).unwrap();
    println!("Message decrypted!");
    
    // Prüfen ob gleich
    if shared_secret_sender == shared_secret_receiver {
        println!("SUCCESS: Quantum-safe encryption works!");
    }
}
