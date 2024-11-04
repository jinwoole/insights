use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use chrono::{Utc, Duration};
use std::error::Error;
use totp_rs::{TOTP, Algorithm};
use base32::{Alphabet, encode};
use qrcode::QrCode;
use image::Luma;
use base64::encode as base64_encode;
use rand;

lazy_static! {
    static ref OTP_STORE: Mutex<HashMap<String, (String, chrono::DateTime<Utc>)>> = Mutex::new(HashMap::new());
}

pub fn generate_otp(user_id: &str) -> String {
    let otp = format!("{:06}", rand::random::<u32>() % 1_000_000);
    let expiry = Utc::now() + Duration::minutes(5);
    let mut store = OTP_STORE.lock().unwrap();
    store.insert(user_id.to_string(), (otp.clone(), expiry));
    otp
}

pub fn verify_otp(user_id: &str, otp: &str) -> bool {
    let mut store = OTP_STORE.lock().unwrap();
    if let Some((stored_otp, expiry)) = store.get(user_id) {
        if stored_otp == otp && Utc::now() < *expiry {
            store.remove(user_id); // Invalidate OTP after successful verification
            return true;
        }
    }
    false
}

pub fn generate_totp_secret() -> String {
    // Generate a 32-byte random secret encoded in base32
    let secret: [u8; 32] = rand::random();
    encode(Alphabet::RFc4648 { padding: false }, &secret)
}

pub fn generate_totp_qr_url(username: &str, secret: &str) -> Result<String, Box<dyn Error>> {
    // Create TOTP instance
    let decoded_secret = base32::decode(Alphabet::Rfc4648 { padding: false }, secret)
        .ok_or("Failed to decode secret")?;
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        decoded_secret,
    )?;

    // Generate the provisioning URI
    let uri = totp.get_url(username);

    // Generate QR code
    let qr = QrCode::new(uri)?;
    let image = qr.render::<Luma<u8>>().build();

    // Convert QR code image to base64 string
    let mut buffer = Vec::new();
    image::DynamicImage::ImageLuma8(image).write_to(
        &mut std::io::Cursor::new(&mut buffer),
        image::ImageOutputFormat::Png,
    )?;
    let encoded_image = base64::engine::general_purpose::STANDARD.encode(&buffer);

    // Return data URL
    Ok(format!("data:image/png;base64,{}", encoded_image))
}

pub fn verify_totp_code(secret: &str, code: &str) -> bool {
    // Decode the secret
    let decoded_secret = base32::decode(RFC4648 { padding: false }, secret);
    if let Some(secret_bytes) = decoded_secret {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes,
        )?;
        totp.check_current(code).unwrap_or(false)
    } else {
        false
    }
}
