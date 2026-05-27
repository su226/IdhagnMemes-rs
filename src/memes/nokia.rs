use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::encode_png;
use meme_generator_utils::image::ImageExt;
use meme_generator_utils::text::Text2Image;
use meme_generator_utils::text_params;
use meme_generator_utils::tools::{load_image, local_date};
use skia_safe::textlayout::TextAlign;
use skia_safe::{Color, Paint, Point, TileMode, image_filters};

use crate::options::NoOptions;
use crate::register_meme;

fn nokia(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("idhagnmemes/nokia/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.rotate(15.0, Some(Point::new(272.0, 275.0)));
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(24, 53, 4));
    // 此处 TileMode 必须是 Decal，否则边缘会出现奇怪的粘连
    paint.set_image_filter(
        image_filters::blur((1.0, 1.0), TileMode::Decal, None, None)
            .ok_or_else(|| Error::MemeFeedback("内部错误: 初始化滤镜失败".to_string()))?,
    );
    let mut text2img = Text2Image::from_text(
        text,
        42.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Left,
            paint = paint,
        ),
    );
    text2img.layout(320.0);
    let height = text2img.height();
    if height > 225.0 {
        return Err(Error::TextOverLength(text.to_string()));
    }
    text2img.draw_on_canvas(canvas, (110, 165));
    encode_png(surface.image_snapshot())
}

register_meme!(
    "nokia1",
    nokia,
    min_texts = 1,
    max_texts = 1,
    keywords = &["无内鬼"],
    default_texts = &["有内鬼\n终止交易"],
    date_created = local_date(2022, 2, 14),
    date_modified = local_date(2026, 5, 27),
);
