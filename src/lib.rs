use wasm_bindgen::prelude::*;

use base64;

use authenticator_format::MigrationPayload;
use prost::Message;
use rqrr::PreparedImage;

mod authenticator_format;

#[wasm_bindgen]
pub fn qr_decode(width: u32, height: u32, mut src: &[u8]) -> Result<String, JsValue> {
    if src.len() != (width * height * 4) as usize {
        return Err("Is this even an image?".to_string().into());
    }

    let mut img = PreparedImage::prepare_from_greyscale(
        width as usize,
        height as usize,
        // Convert the [RGBA] input to gray on the fly by using simple
        // average.
        // XXX I'm assuming this is called in the "normal" order:
        //     x->width, then y->height
        move |_x, _y| {
            let luminance = if let [red, green, blue, _alpha] = &src[..4] {
                let sum = *red as usize + *green as usize + *blue as usize;
                (sum as f32 / 3.0) as u8
            } else {
                // Shouldn't happen, so... panic maybe?
                0
            };
            src = &src[4..];
            luminance
        },
    );

    let grids = img.detect_grids();

    if grids.len() == 1 {
        grids[0]
            .decode()
            .map(|(_meta, content)| content)
            .map_err(|err| err.to_string().into())
    } else {
        Err(format!("Found {} grids", grids.len()).into())
    }
}

#[wasm_bindgen]
pub fn decode_otpauth(input: String) -> Result<String, JsValue> {
    nojs_otpauth_decode(input)
        .map_err(|err| err.into())
        .map(|uris| uris.join("\n"))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OtpError {
    BadInput,
    Base64Decode,
    ProtoDecode,
    UrlDecode,
}

impl From<OtpError> for JsValue {
    fn from(src: OtpError) -> Self {
        match src {
            OtpError::BadInput => JsValue::from_str("Not otpauth-migration://"),
            OtpError::Base64Decode => JsValue::from_str("Decoding proto message failed"),
            OtpError::ProtoDecode => JsValue::from_str("base64decode error"),
            OtpError::UrlDecode => JsValue::from_str("url-decode failed"),
        }
    }
}

fn nojs_otpauth_decode(input: String) -> Result<Vec<String>, OtpError> {
    let urldecoded = urlencoding::decode(&input).map_err(|_err| OtpError::UrlDecode)?;

    let b64_encoded = urldecoded
        .strip_prefix("otpauth-migration://offline?data=")
        .ok_or(OtpError::BadInput)?;

    if let Ok(decoded) = base64::decode(b64_encoded.trim()) {
        let message =
            MigrationPayload::decode(&decoded[..]).map_err(|_err| OtpError::ProtoDecode)?;
        Ok(migration_to_uris(&message))
    } else {
        Err(OtpError::Base64Decode)
    }
}

// Tries to dump the payload into something that respects:
// https://github.com/google/google-authenticator/wiki/Key-Uri-Format
fn migration_to_uris(payload: &MigrationPayload) -> Vec<String> {
    let mut uris = Vec::new();

    // XXX This will likely die horribly every time I forget to
    //     url-encode...
    for param in payload.otp_parameters.iter() {
        let mut uri = String::from("otpauth://");

        use authenticator_format::migration_payload::OtpType;
        match param.r#type() {
            OtpType::Unspecified => todo!("I think this only exists because protobuf..."),
            OtpType::Hotp => uri.push_str("hotp/"),
            OtpType::Totp => uri.push_str("totp/"),
        }

        uri.push_str(&urlencoding::encode(&param.name));
        uri.push_str("?secret=");
        uri.push_str(&secret_as_rfc3548(&param.secret));

        if !param.issuer.is_empty() {
            uri.push_str("&issuer=");
            uri.push_str(&urlencoding::encode(&param.issuer));
        }

        use authenticator_format::migration_payload::Algorithm;
        match param.algorithm() {
            Algorithm::Sha1 => uri.push_str("&algorithm=SHA1"),
            Algorithm::Sha256 => uri.push_str("&algorithm=SHA256"),
            Algorithm::Sha512 => uri.push_str("&algorithm=SHA512"),
            Algorithm::Md5 => uri.push_str("&algorithm=MD5"),
            // Unspecified is ignored
            Algorithm::Unspecified => {}
        }

        use authenticator_format::migration_payload::DigitCount;
        match param.digits() {
            DigitCount::Six => uri.push_str("&digits=6"),
            DigitCount::Eight => uri.push_str("&digits=8"),
            // Unspecified is ignored
            DigitCount::Unspecified => {}
        }

        // fudging goggle authenticator ignores "period" and apparently
        // doesn't export it either?
        // It's optional tho, so...
        // There's this left-over `counter` i64

        uris.push(uri);
    }

    uris
}

fn secret_as_rfc3548(secret: &[u8]) -> String {
    base32::encode(base32::Alphabet::RFC4648 { padding: false }, secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    use image::Pixel;

    #[test]
    fn can_decode_rgba() {
        let img = image::open("fudgegoggle.png").expect("input is an actual image");

        let rgba = img.as_rgba8().expect("image can be converted to RGBA");
        let mut bytes = Vec::new();

        for pixel in rgba.pixels() {
            bytes.extend_from_slice(pixel.channels());
        }

        let res = qr_decode(rgba.width(), rgba.height(), &bytes);
        assert_eq!(Ok("fudgegoggle".to_string()), res);
    }

    #[test]
    fn can_decode_otpauth() {
        let input = std::fs::read_to_string("otpauth-sample.txt").unwrap();
        let expected = String::from(
            "otpauth://totp/pi%40raspberrypi?secret=7KSQL2JTUDIS5EF65KLMRQIIGY&issuer=raspberrypi&algorithm=SHA1&digits=6"
            );
        assert_eq!(Ok(vec![expected]), nojs_otpauth_decode(input));
    }
}
