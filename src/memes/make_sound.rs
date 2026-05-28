use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::canvas::CanvasExt;
use meme_generator_utils::encoder::encode_png;
use meme_generator_utils::image::{Fit, ImageExt};
use meme_generator_utils::tools::{load_image, local_date, new_surface};
use skia_safe::{Color, Rect};

use crate::options::NoOptions;
use crate::register_meme;

fn make_sound(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("idhagnmemes/make_sound/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::from_rgb(236, 240, 217));

    let dialogue_image = if !images.is_empty() {
        Some(&images[0])
    } else {
        None
    };
    let dialogue_text = if texts.len() > 1 {
        Some(&texts[1])
    } else {
        None
    };
    let caption = &texts[0];

    match (dialogue_image, dialogue_text) {
        (Some(_), Some(_)) => return Err(Error::MemeFeedback("不能同时包含文本和图片".into())),
        (Some(image), None) => {
            canvas.draw_image(image.image.resize_fit((122, 74), Fit::Cover), (6, 26), None);
            canvas.draw_image(frame, (0, 0), None);
        }
        (None, Some(text)) => {
            canvas.draw_image(frame, (0, 0), None);
            canvas.draw_text_area_auto_font_size(
                Rect::from_xywh(6.0, 26.0, 122.0, 74.0),
                text,
                20.0,
                50.0,
                None,
            )?;
        }
        _ => return Err(Error::MemeFeedback("需要包含文本或图片".into())),
    };

    canvas.draw_text_area_auto_font_size(
        Rect::from_xywh(0.0, 310.0, 380.0, 70.0),
        caption,
        20.0,
        50.0,
        None,
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "make_sound",
    make_sound,
    min_images = 0,
    max_images = 1,
    min_texts = 1,
    max_texts = 2,
    keywords = &["发出声音"],
    default_texts = &["发出猛男的声音", "嘤"],
    date_created = local_date(2026, 5, 28),
    date_modified = local_date(2026, 5, 28),
);
