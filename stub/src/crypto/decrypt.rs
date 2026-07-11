use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305,
    Nonce,
};

pub fn decrypt_payload(key: &[u8; 32], nonce: &[u8; 12], encrypted_payload: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce_array = Nonce::from_slice(nonce);

    let decrypted = cipher
        .decrypt(&nonce_array, encrypted_payload)
        .map_err(|e| format!("Error de integridad/descifrado: {:?}", e))?;

    Ok(decrypted)
}
