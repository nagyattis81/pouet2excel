use flate2::read::GzDecoder;
use std::{fs::File, io::copy, path::Path};

pub fn decompress_gzip_file(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> std::io::Result<()> {
    let input = File::open(src)?;
    let mut decoder = GzDecoder::new(input);
    let mut output = File::create(dest)?;
    copy(&mut decoder, &mut output)?;
    Ok(())
}
