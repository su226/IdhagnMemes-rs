use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::canvas::CanvasExt;
use meme_generator_utils::encoder::encode_png;
use meme_generator_utils::image::ImageExt;
use meme_generator_utils::text_params;
use meme_generator_utils::tools::{load_image, local_date, new_paint};
use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::{Color, FontStyle, IRect};

use crate::options::NoOptions;
use crate::register_meme;

fn addict(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let image = load_image("idhagnmemes/addict/0.png")?;
    let mut surface = image.to_surface();
    let canvas = surface.canvas();

    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(398, 648, 688, 720),
        &texts[0],
        20.0,
        50.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            font_style = FontStyle::new(Weight::MEDIUM, Width::NORMAL, Slant::Upright),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "addict",
    addict,
    min_texts = 1,
    max_texts = 1,
    keywords = &["成瘾前后", "成瘾"],
    default_texts = &["表情包制作"],
    date_created = local_date(2022, 7, 28),
    date_modified = local_date(2026, 5, 22),
);
