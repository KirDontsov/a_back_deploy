use aes::Aes256;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use hex;
use rand_core::{OsRng, RngCore};
use std::error::Error;
use std::fmt;

// Custom error type to handle different error types
#[derive(Debug)]
pub struct EncryptionError {
	message: String,
}

impl fmt::Display for EncryptionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl Error for EncryptionError {}

pub fn encrypt_data(data: &str, key: &[u8; 32], iv: &[u8; 16]) -> String {
	type Aes256CbcEnc = Encryptor<Aes256>;

	let cipher = Aes256CbcEnc::new_from_slices(key, iv).expect("Invalid key or IV length");
	let plaintext = data.as_bytes();

	// Pad the plaintext to a multiple of block size using PKCS7
	let block_size = 16;
	let padding_len = block_size - (plaintext.len() % block_size);
	let mut padded_plaintext = plaintext.to_vec();
	padded_plaintext.resize(plaintext.len() + padding_len, padding_len as u8);

	let ciphertext =
		cipher.encrypt_padded_vec_mut::<cipher::block_padding::Pkcs7>(&padded_plaintext);
	hex::encode(ciphertext)
}

pub fn decrypt_data(
	encrypted_data: &str,
	key: &[u8; 32],
	iv: &[u8; 16],
) -> Result<String, Box<dyn std::error::Error>> {
	type Aes256CbcDec = Decryptor<Aes256>;

	let ciphertext = hex::decode(encrypted_data)?;
	let cipher = Aes256CbcDec::new_from_slices(key, iv).expect("Invalid key or IV length");

	// Handle the decryption result properly
	match cipher.decrypt_padded_vec_mut::<cipher::block_padding::Pkcs7>(&ciphertext) {
		Ok(mut decrypted) => {
			// Remove PKCS7 padding manually if needed
			if !decrypted.is_empty() {
				let padding_len = decrypted[decrypted.len() - 1] as usize;
				if padding_len <= decrypted.len()
					&& decrypted
						.iter()
						.skip(decrypted.len() - padding_len)
						.all(|&x| x == padding_len as u8)
				{
					decrypted.truncate(decrypted.len() - padding_len);
				}
			}
			Ok(String::from_utf8(decrypted)?)
		}
		Err(e) => Err(Box::new(EncryptionError {
			message: format!("Decryption failed: {:?}", e),
		})),
	}
}

pub fn generate_iv() -> [u8; 16] {
	let mut iv = [0u8; 16];
	OsRng.fill_bytes(&mut iv);
	iv
}

// Function to decrypt Avito credentials
pub fn decrypt_avito_credentials(
	encrypted_secret: &str,
	encrypted_client_id: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
	// Global key for encryption (in production, this should be stored securely)
	static ENCRYPTION_KEY: [u8; 32] = [
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
		26, 27, 28, 29, 30, 31, 32,
	];

	// Split IV and encrypted data for secret
	let (iv_secret, encrypted_secret_data) = split_iv_and_data(encrypted_secret)?;
	let secret = decrypt_data(&encrypted_secret_data, &ENCRYPTION_KEY, &iv_secret)?;

	// Split IV and encrypted data for client_id
	let (iv_client_id, encrypted_client_id_data) = split_iv_and_data(encrypted_client_id)?;
	let client_id = decrypt_data(&encrypted_client_id_data, &ENCRYPTION_KEY, &iv_client_id)?;

	Ok((secret, client_id))
}

// Function to combine IV and encrypted data
pub fn combine_iv_and_data(iv: &[u8; 16], encrypted_data: &str) -> String {
	format!("{}:{}", hex::encode(iv), encrypted_data)
}

// Function to split IV and encrypted data
pub fn split_iv_and_data(
	combined_data: &str,
) -> Result<([u8; 16], String), Box<dyn std::error::Error>> {
	let parts: Vec<&str> = combined_data.split(':').collect();
	if parts.len() != 2 {
		return Err("Invalid combined data format".into());
	}

	let iv = hex::decode(parts[0])?;
	if iv.len() != 16 {
		return Err("Invalid IV length".into());
	}

	let mut iv_bytes = [0u8; 16];
	iv_bytes.copy_from_slice(&iv);

	Ok((iv_bytes, parts[1].to_string()))
}
