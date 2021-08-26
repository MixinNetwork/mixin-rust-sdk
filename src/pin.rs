use aes::{Aes256, BLOCK_SIZE};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use byteorder::{ByteOrder, LittleEndian};
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha512};
use std::error;
use std::time::SystemTime;
use x25519_dalek;

type AesCbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(
    pin: &str,
    pin_token_base64: &str,
    sid: &str,
    private_base64: &str,
    iterator: u64,
) -> Result<String, Box<error::Error>> {
    let private_bytes = base64::decode_config(private_base64, base64::URL_SAFE_NO_PAD)?;
    let public_bytes = base64::decode_config(pin_token_base64, base64::URL_SAFE_NO_PAD)?;

    let mut public: [u8; 32] = [0u8; 32];
    public.copy_from_slice(public_bytes.as_slice());
    let shared_key = x25519_dalek::x25519(curve25519(private_bytes.as_slice()), public);

    let mut pin_buf: Vec<u8> = vec![];
    pin_buf.extend_from_slice(pin.as_bytes());
    let mut time_buf = [0u8; 8];
    LittleEndian::write_u64(
        &mut time_buf,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
    );
    pin_buf.extend_from_slice(&time_buf);

    let mut iterator_buf = [0u8; 8];
    LittleEndian::write_u64(&mut iterator_buf, iterator);
    pin_buf.extend_from_slice(&iterator_buf);

    let padding = BLOCK_SIZE - pin_buf.len() % BLOCK_SIZE;
    let mut padding_buf: Vec<u8> = [padding as u8].repeat(padding);
    pin_buf.append(&mut padding_buf);

    let mut iv = [0u8, BLOCK_SIZE];
    thread_rng().fill(&mut arr[..]);

    Ok(String::from(""))
}

fn curve25519(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();

    let mut private: [u8; 32] = [0u8; 32];
    private.copy_from_slice(&result.as_slice()[..32]);
    private
}
