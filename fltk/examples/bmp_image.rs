use fltk::{app, frame::Frame, image::BmpImage, prelude::*, window::Window};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const IMAGE: &[u8] = include_bytes!("fltk.bmp");

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("Image");

    wind.begin();

    let mut frame = Frame::default_fill();
    let image = BmpImage::from_data(IMAGE)?;
    frame.set_image_scaled(Some(image));

    wind.end();
    wind.show();

    app.run()?;
    Ok(())
}
