use image::io::Reader;
use std::io::{Cursor, Read, Write};

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut buf: Vec<u8> = vec![];
    stdin.read_to_end(&mut buf).unwrap();

    if buf.is_empty() {
        println!("No image provided");
        return;
    }

    let data = Cursor::new(buf);
    let reader = Reader::new(data)
        .with_guessed_format()
        .expect("Failed to read data");
    let format = reader.format().expect("Failed to detect image format");
    let img = reader.decode().expect("Failed to decode image");

    let img = if img.width() > img.height() {
        let delta = img.width() - img.height();
        img.crop_imm(delta / 2, 0, img.height(), img.height())
    } else if img.height() > img.width() {
        let delta = img.height() - img.width();
        img.crop_imm(0, delta / 2, img.width(), img.width())
    } else {
        img
    };

    const THUMBNAIL_SIZE: u32 = 64;
    let img = img.resize(
        THUMBNAIL_SIZE,
        THUMBNAIL_SIZE,
        image::imageops::FilterType::Lanczos3,
    );

    let out_buf: Vec<u8> = vec![];
    let mut writer = Cursor::new(out_buf);
    img.write_to(&mut writer, format)
        .expect("Error writing image to buffer");
    std::io::stdout()
        .write_all(writer.get_ref())
        .expect("Error writing buffer to stdout");
}
