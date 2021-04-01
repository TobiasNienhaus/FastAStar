use crate::Graph;
use image::{io::Reader as ImgReader, GrayImage, Pixel};
use std::io::Result as IoResult;
use std::path::PathBuf;

fn load_luma8(name: PathBuf) -> IoResult<GrayImage> {
    Ok(ImgReader::open(name)?
        .with_guessed_format()?
        .decode()
        .unwrap()
        .to_luma8())
}

pub fn test_img(name: PathBuf) -> IoResult<()> {
    let img = load_luma8(name)?;
    for (_, row) in img.rows().enumerate() {
        for (_, pix) in row.enumerate() {
            for channel in pix.channels() {
                print!("{:^3} ", channel);
            }
        }
        println!();
    }
    Ok(())
}
