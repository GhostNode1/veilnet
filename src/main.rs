use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use oqs::kem;

#[tokio::main]
async fn main() {
    println!("VEILNET — Quantum Safe Network");
    println!("GhostNode1 online.");
    println!("Starting node on port 8888...");

    let listener = TcpListener::bind("0.0.0.0:8888").await.unwrap();
    println!("Listening for connections...");

    let kem = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
    let (public_key, _secret_key) = kem.keypair().unwrap();
    println!("Quantum-safe keys ready!");
    println!("Waiting for peer...");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Peer connected: {}", addr);
        socket.write_all(b"VEILNET_HELLO").await.unwrap();
    }
}
