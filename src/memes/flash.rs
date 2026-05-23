use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::make_png_or_gif;
use meme_generator_utils::image::ImageExt;
use meme_generator_utils::tools::{load_image, local_date};
use skia_safe::{Image, SamplingOptions};

use crate::options::NoOptions;
use crate::register_meme;

fn flash(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let flash = load_image("idhagnmemes/flash/0.png")?;

    make_png_or_gif(images, |images: Vec<Image>| {
        let image =
            images[0].resize_exact_with_sampling_options((8, 6), SamplingOptions::default());
        let image = image.resize_exact_with_sampling_options(
            (image.width() * 50, image.height() * 50),
            SamplingOptions::default(),
        );
        let image = image.brightness(0.5);
        let mut surface = image.to_surface();
        let surface_width = surface.width();
        let surface_height = surface.height();
        let canvas = surface.canvas();
        canvas.draw_image(
            &flash,
            (
                (surface_width - flash.width()) / 2,
                (surface_height - flash.height()) / 2,
            ),
            None,
        );
        Ok(surface.image_snapshot())
    })
}

register_meme!(
    "flash",
    flash,
    min_images = 1,
    max_images = 1,
    keywords = &["闪照"],
    date_created = local_date(2022, 11, 15),
    date_modified = local_date(2026, 5, 23),
);
