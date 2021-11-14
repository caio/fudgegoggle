use wasm_bindgen::prelude::*;

use rqrr::PreparedImage;

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
}
