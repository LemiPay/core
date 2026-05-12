use alloy::signers::SignerSync;
use alloy::signers::local::PrivateKeySigner;
use std::io;

fn main() {
    let signer = PrivateKeySigner::random();
    let address = signer.address().to_string();

    println!("=======================================================");
    println!("🏦 BILLETERA DE PRUEBA GENERADA");
    println!("Address: {}", address);
    println!("=======================================================\n");
    println!(
        "1️⃣ Copiá este Address y usalo en Postman para pedir el Challenge (/request-challenge)."
    );
    println!("2️⃣ Cuando Postman te devuelva el Nonce, pegalo acá abajo.\n");

    let mut email = String::new();
    let mut nonce = String::new();

    println!("📧 Ingresá el Email (el mismo que usaste en Postman):");
    io::stdin().read_line(&mut email).unwrap();
    println!("🔑 Ingresá el Nonce que te dio Postman:");
    io::stdin().read_line(&mut nonce).unwrap();

    let email = email.trim().to_lowercase();
    let nonce = nonce.trim();

    let message = format!(
        "Bienvenido a LemiPay.\n\n\
        Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
        Email: {}\n\
        Nonce: {}",
        email, nonce
    );

    let signature = signer.sign_message_sync(message.as_bytes()).unwrap();
    let signature_hex = format!("0x{}", alloy::hex::encode(signature.as_bytes()));

    println!("\n=======================================================");
    println!("🚀 PAYLOAD LISTO PARA /verify-challenge");
    println!("Copiá este JSON y pegalo en el Body de Postman:");
    println!("=======================================================\n");
    println!(
        r#"{{
  "email": "{}",
  "address": "{}",
  "signature": "{}",
  "nonce": "{}"
}}"#,
        email, address, signature_hex, nonce
    );
    println!("\n=======================================================");
}
