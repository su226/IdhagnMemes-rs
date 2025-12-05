use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn ori(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("idhagnmemes/ori/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_exact((100, 100)).circle();
        canvas.draw_image(&img, (305, 222), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "ori",
    ori,
    min_images = 1,
    max_images = 1,
    keywords = &["ori", "拥抱光明"],
    date_created = local_date(2022, 11, 11),
    date_modified = local_date(2025, 12, 5),
);
