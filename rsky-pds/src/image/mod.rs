use anyhow::Result;
use image::io::Reader as ImageReader;
use image::{guess_format, GenericImageView};
use std::io::Cursor;

/// A struct representing the structure of an image
///
/// # Examples:
/// ```rust
/// use rsky_pds::image::ImageInfo; 
/// let image = ImageInfo {
///     height: 512,
///     width: 512,
///     size: Option::from(1000000u32),
///     mime: String::from("image/png")};
/// ```
///
/// This struct does not validate input values. Be sure that `height`, `width`, and `size`
/// are within the range [0, 2^(32)-1], and that mime is given a string.
pub struct ImageInfo {
    /// the height of the image
    pub height: u32,
    /// the width of the image
    pub width: u32,
    /// the amount of memory (in bytes) the image takes up
    pub size: Option<u32>,
    /// the media type of the image
    pub mime: String,
}

/// Returns the mime-type of the file.
pub async fn mime_type_from_bytes(bytes: Vec<u8>) -> Result<Option<String>> {
    match infer::get(bytes.as_slice()) {
        Some(kind) => Ok(Some(kind.mime_type().to_string())),
        None => Ok(None),
    }
}

/// Returns the full image details
pub async fn maybe_get_info(bytes: Vec<u8>) -> Result<Option<ImageInfo>> {
    let process_image = || -> Result<Option<ImageInfo>> {
        let img = ImageReader::new(Cursor::new(bytes.clone()))
            .with_guessed_format()?
            .decode()?;
        let (width, height) = img.dimensions();
        let mime = guess_format(bytes.as_slice())?.to_mime_type().to_string();
        let size: Option<u32> = None;
        Ok(Some(ImageInfo {
            height,
            width,
            size,
            mime,
        }))
    };

    return match process_image() {
        Ok(info) => Ok(info),
        Err(_) => Ok(None),
    };
}
