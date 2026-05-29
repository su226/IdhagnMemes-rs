use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::encode_png;
use meme_generator_utils::text::{Text2Image, TextParams};
use meme_generator_utils::text_params;
use meme_generator_utils::tools::{load_image, local_date, new_surface};
use skia_safe::{Color, FontStyle};

use crate::options::NoOptions;
use crate::register_meme;

const PADDING_X: i32 = 45;
const PADDING_Y: i32 = 25;
const MEDAL_MARGIN_RIGHT: i32 = 20;
const MEDAL_MARGIN_BOTTOM: i32 = 40;
const FONT_SIZE: f32 = 60.0;

fn good_answer(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let image = load_image("idhagnmemes/good_answer/0.png")?;
    let (title, content): (&str, &str) = match texts.len() {
        1 => ("优质解答", &texts[0]),
        2 => (&texts[0], &texts[1]),
        _ => unreachable!(),
    };
    let title_layout = Text2Image::from_text(
        title,
        FONT_SIZE,
        text_params!(font_style = FontStyle::bold()),
    );
    let content_layout = Text2Image::from_text(content, FONT_SIZE, TextParams::default());
    let width = PADDING_X * 2
        + (image.width() + MEDAL_MARGIN_RIGHT + (title_layout.longest_line().ceil() as i32))
            .max(content_layout.longest_line().ceil() as i32);
    let height = PADDING_Y * 2
        + image.height().max(title_layout.height().ceil() as i32)
        + MEDAL_MARGIN_BOTTOM
        + (content_layout.height().ceil() as i32);
    let mut surface = new_surface((width, height));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(&image, (PADDING_X, PADDING_Y), None);
    title_layout.draw_on_canvas(
        canvas,
        (
            PADDING_X + image.width() + MEDAL_MARGIN_RIGHT,
            PADDING_Y + ((image.height() as f32 - title_layout.height()) / 2.0) as i32,
        ),
    );
    content_layout.draw_on_canvas(
        canvas,
        (PADDING_X, PADDING_Y + image.height() + MEDAL_MARGIN_BOTTOM),
    );
    encode_png(surface.image_snapshot())
}

register_meme!(
    "good_answer",
    good_answer,
    min_texts = 1,
    max_texts = 2,
    keywords = &["优质解答"],
    default_texts = &["优质解答", "我不知道"],
    date_created = local_date(2026, 5, 29),
    date_modified = local_date(2026, 5, 29),
);
