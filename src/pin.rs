use aes::{Aes256, BLOCK_SIZE};
use block_modes::block_padding;
use block_modes::{BlockMode, Cbc};
use byteorder::{ByteOrder, LittleEndian};
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha512};
use std::error;
use std::time::SystemTime;
use x25519_dalek;

type Aes256Cbc = Cbc<Aes256, block_padding::ZeroPadding>;

pub fn encrypt(
    pin: &str,
    iterator: u64,
    pin_token_base64: &str,
    private_base64: &str,
) -> Result<String, Box<dyn error::Error>> {
    let private_bytes = base64::decode_config(private_base64, base64::URL_SAFE_NO_PAD)?;
    let public_bytes = base64::decode_config(pin_token_base64, base64::URL_SAFE_NO_PAD)?;

    let mut public: [u8; 32] = [0u8; 32];
    public.copy_from_slice(public_bytes.as_slice());
    let shared_key = x25519_dalek::x25519(curve25519(private_bytes.as_slice()), public);

    let mut pin_buf: Vec<u8> = vec![];
    pin_buf.extend_from_slice(pin.as_bytes());
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let mut time_buf = [0u8; 8];
    LittleEndian::write_u64(&mut time_buf, now);
    pin_buf.extend_from_slice(&time_buf);

    let mut iterator_buf = [0u8; 8];
    LittleEndian::write_u64(&mut iterator_buf, iterator);
    pin_buf.extend_from_slice(&iterator_buf);

    let padding = BLOCK_SIZE - pin_buf.len() % BLOCK_SIZE;
    let mut padding_buf: Vec<u8> = [padding as u8].repeat(padding);
    pin_buf.append(&mut padding_buf);

    let mut iv = [0u8; BLOCK_SIZE];
    thread_rng().fill(&mut iv[..]);

    let cipher = Aes256Cbc::new_from_slices(&shared_key, &iv).unwrap();
    let mut ciphertext = cipher.encrypt_vec(&mut pin_buf);

    let mut encrypted_pin_buf: Vec<u8> = vec![];
    encrypted_pin_buf.extend_from_slice(&iv);
    encrypted_pin_buf.append(&mut ciphertext);

    Ok(base64::encode_config(
        encrypted_pin_buf,
        base64::URL_SAFE_NO_PAD,
    ))
}

fn curve25519(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha512::new();
    hasher.update(&input[..32]);
    let result = hasher.finalize();

    let mut private: [u8; 32] = [0u8; 32];
    private.copy_from_slice(&result.as_slice()[..32]);
    private
}
