use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::make_png_or_gif;
use meme_generator_utils::image::{Fit, ImageExt};
use meme_generator_utils::tools::{load_image, local_date};
use skia_safe::Image;

use crate::image::flatten;
use crate::options::NoOptions;
use crate::register_meme;

fn patrick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("idhagnmemes/patrick/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = flatten(&&images[0].resize_fit((280, 280), Fit::Cover)).round_corner(60.0);
        canvas.draw_image(&img, (403, 319), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "patrick",
    patrick,
    min_images = 1,
    max_images = 1,
    keywords = &["派大星举"],
    date_created = local_date(2026, 6, 7),
    date_modified = local_date(2026, 6, 7),
);
