use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::make_png_or_gif;
use meme_generator_utils::image::{Fit, ImageExt};
use meme_generator_utils::tools::{load_image, local_date};
use skia_safe::Image;

use crate::options::NoOptions;
use crate::register_meme;

fn netease_mc(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("idhagnmemes/netease_mc/0.png")?;

    let func = |images: Vec<Image>| {
        let image = images[0].resize_fit((512, 512), Fit::Cover);
        let mut surface = image.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "netease_mc",
    netease_mc,
    min_images = 1,
    max_images = 1,
    keywords = &["贺新春"],
    date_created = local_date(2025, 3, 21),
    date_modified = local_date(2026, 5, 25),
);
