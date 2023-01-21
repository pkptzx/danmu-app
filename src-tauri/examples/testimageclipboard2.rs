use std::io::Cursor;
use image::{io::Reader as ImageReader, ImageFormat, ImageBuffer};
use arboard::{Clipboard, ImageData};
use anyhow::{Result, Ok};
fn main() -> Result<()>{
    // let imgBase64 = "data:image/png;base64,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";
    // let imgBase64 = &imgBase64[22..];
    // println!("{imgBase64}");
    // let bytes = base64::decode(&imgBase64)?;
    // // let bs = bytes.as_slice();

    // let mut img2 = ImageReader::new(Cursor::new(&bytes));
    // img2.set_format(ImageFormat::Png);
    // let fmt =img2.format();
    // println!("fmt: {:?}", fmt);
    // let img  = img2.decode()?;
    
    // // println!("{img2}");
    // img.save("empty.png")?;

    let img = ImageReader::open(r"C:\Users\loyal\Desktop\xxx.png").unwrap().decode().unwrap();

    let mut ctx = Clipboard::new().unwrap();

    // let image = image::ImageBuffer::from_fn(8, 8, |x, y| {
    //     if (x * y) % 7 == 0 {
    //         image::Rgba([0, 255, 0, 255])
    //     } else {
    //         image::Rgba([0, 0, 0, 255])
    //     }
    // });
    // bytes.as_ref().into()
    // image.as_raw().into()
    let rgba8 = img.into_rgba8(); // Of course you can use `img.clone().into_rgba8()` if needed
    let (w,h) = rgba8.dimensions();
    let bytes = rgba8.into_raw();
	let img_data = ImageData { width: w as usize, height: h as usize,  bytes: bytes.into() };
	ctx.set_image(img_data).unwrap();

    
    Ok(())
}


 