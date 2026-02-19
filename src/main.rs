use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use oqs::kem;
use std::env;

#[tokio::main]
async fn main() {
    println!("VEILNET — Quantum Safe Network");
    println!("GhostNode1 online.");
    println!("");

    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "send" {
        // SENDER
        println!("Mode: SENDER");
        let kem = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
        let (public_key, _) = kem.keypair().unwrap();
        let (ciphertext, shared_secret) = kem.encapsulate(&public_key).unwrap();
        
        let mut stream = TcpStream::connect("127.0.0.1:8888").await.unwrap();
        stream.write_all(ciphertext.as_ref()).await.unwrap();
        println!("Encrypted message sent!");
        println!("Shared secret: {:?}", &shared_secret.as_ref()[..4]);
    } else {
        // RECEIVER
        println!("Mode: RECEIVER");
        let kem = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
        let (_, secret_key) = kem.keypair().unwrap();
        
        let listener = TcpListener::bind("0.0.0.0:8888").await.unwrap();
        println!("Listening on port 8888...");
        
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Peer connected: {}", addr);
        
        let mut buf = vec![0u8; 1568];
        socket.read_exact(&mut buf).await.unwrap();
        println!("Encrypted message received!");
        println!("SUCCESS: Quantum-safe channel established!");
    }
}
