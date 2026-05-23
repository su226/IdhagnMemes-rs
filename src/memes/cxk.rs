use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::encode_png;
use meme_generator_utils::image::ImageExt;
use meme_generator_utils::tools::{load_image, local_date, new_surface};
use rand::RngExt;
use skia_safe::Color;

use crate::options::NoOptions;
use crate::register_meme;

fn cxk(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut rng = rand::rng();

    let frame = load_image("idhagnmemes/cxk/0.png")?;
    let image0 = &images[0].image.resize_exact((130, 130));
    let image1 = &images[1]
        .image
        .resize_exact((130, 130))
        .rotate_crop(rng.random_range(0.0..360.0));

    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(image0, (382, 59), None);
    canvas.draw_image(image1, (609, 317), None);
    canvas.draw_image(frame, (0, 0), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "cxk",
    cxk,
    min_images = 2,
    max_images = 2,
    keywords = &["蔡徐坤", "cxk", "打篮球", "鸡你太美"],
    date_created = local_date(2022, 2, 14),
    date_modified = local_date(2026, 5, 22),
);
