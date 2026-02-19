use oqs::kem;

fn main() {
    println!("VEILNET — Quantum Safe Network");
    println!("GhostNode1 online.");
    println!("Initializing quantum-safe encryption...");
    
    let kem = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
    let (public_key, secret_key) = kem.keypair().unwrap();
    
    println!("Quantum-safe keys generated!");
    println!("Public key length: {} bytes", public_key.as_ref().len());
    println!("VEILNET node ready.");
}
