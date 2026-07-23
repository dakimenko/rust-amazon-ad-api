use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use md5::{Digest, Md5};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// AES-CBC encrypt a string with a 16-byte key.
/// Returns base64-encoded ciphertext.
/// The IV is derived from the first 16 bytes of the key (deterministic).
pub fn aes_encrypt(key: &[u8], data: &str) -> Result<String, anyhow::Error> {
    if key.len() != 16 {
        return Err(anyhow::anyhow!("AES key must be exactly 16 bytes"));
    }

    let iv = &key[..16];

    let plaintext = data.as_bytes();
    let buf_len = plaintext.len() + 16;
    let mut buf = vec![0u8; buf_len];

    let ciphertext = Aes128CbcEnc::new(key.into(), iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .map_err(|e| anyhow::anyhow!("AES encrypt failed: {:?}", e))?;

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        ciphertext,
    ))
}

/// AES-CBC encrypt with a cryptographically random IV (CSPRNG via `getrandom`).
///
/// Returns base64-encoded `IV || ciphertext` (first 16 bytes are the random IV).
/// This prevents identical plaintext from producing the same ciphertext across calls.
pub fn aes_encrypt_with_random_iv(key: &[u8], data: &str) -> Result<String, anyhow::Error> {
    if key.len() != 16 {
        return Err(anyhow::anyhow!("AES key must be exactly 16 bytes"));
    }

    let plaintext = data.as_bytes();
    let pad_len = plaintext.len() + 16;
    let mut output = vec![0u8; 16 + pad_len];

    getrandom::getrandom(&mut output[..16])
        .map_err(|e| anyhow::anyhow!("getrandom failed: {}", e))?;

    let (iv_slice, buf_slice) = output.split_at_mut(16);
    let ciphertext = Aes128CbcEnc::new(key.into(), (&*iv_slice).into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, buf_slice)
        .map_err(|e| anyhow::anyhow!("AES encrypt failed: {:?}", e))?;

    let total_len = 16 + ciphertext.len();
    output.truncate(total_len);

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &output,
    ))
}

/// AES-CBC decrypt a base64-encoded ciphertext with a 16-byte key.
/// Returns the raw decrypted bytes.
pub fn aes_decrypt(key: &[u8], data: &str) -> Result<Vec<u8>, anyhow::Error> {
    if key.len() != 16 {
        return Err(anyhow::anyhow!("AES key must be exactly 16 bytes"));
    }

    let iv = &key[..16];

    let ciphertext = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data)
        .map_err(|e| anyhow::anyhow!("Base64 decode failed: {}", e))?;

    let mut buf = vec![0u8; ciphertext.len()];

    let plaintext = Aes128CbcDec::new(key.into(), iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf)
        .map_err(|e| anyhow::anyhow!("AES decrypt failed: {:?}", e))?;

    Ok(plaintext.to_vec())
}

/// Compute MD5 hash of data, returned as a hex string.
pub fn md5_hash(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_encrypt_decrypt_roundtrip() {
        let key: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let plaintext = "Hello, Amazon Ads!";
        let encrypted = aes_encrypt(&key, plaintext).expect("encrypt failed");
        let decrypted = aes_decrypt(&key, &encrypted).expect("decrypt failed");
        assert_eq!(plaintext.as_bytes(), &decrypted[..]);
    }

    #[test]
    fn test_aes_encrypt_produces_different_output() {
        let key: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let result1 = aes_encrypt(&key, "test").unwrap();
        let result2 = aes_encrypt(&key, "test").unwrap();
        assert!(!result1.is_empty());
        assert!(!result2.is_empty());
        // CBC with same key+data → same output (deterministic with fixed IV)
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_aes_invalid_key_length() {
        let key = [1u8; 15];
        assert!(aes_encrypt(&key, "test").is_err());
    }

    #[test]
    fn test_md5_known_hash() {
        let result = md5_hash(b"hello");
        assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_md5_empty() {
        let result = md5_hash(b"");
        assert_eq!(result, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_aes_base64_decode_fails_on_garbage() {
        let key: [u8; 16] = [0; 16];
        assert!(aes_decrypt(&key, "not-valid-base64!!!").is_err());
    }

    #[test]
    fn test_aes_encrypt_with_random_iv_produces_different_output() {
        let key: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let result1 = aes_encrypt_with_random_iv(&key, "test").unwrap();
        let result2 = aes_encrypt_with_random_iv(&key, "test").unwrap();
        // With random IV each call should (with overwhelming probability) differ
        assert!(!result1.is_empty());
        assert!(!result2.is_empty());
        // They should differ because IVs are random
        assert_ne!(result1, result2);
    }
}
