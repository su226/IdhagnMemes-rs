use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::make_png_or_gif;
use meme_generator_utils::image::{Fit, ImageExt};
use meme_generator_utils::text::Text2Image;
use meme_generator_utils::text_params;
use meme_generator_utils::tools::{local_date, new_surface};
use skia_safe::{Canvas, Color, IRect, Image, Paint, PaintStyle, PathBuilder};

use crate::options::NoOptions;
use crate::register_meme;

const MARGIN: i32 = 32;
const AVATAR_WIDTH: i32 = 98;
const AVATAR_BORDER_WIDTH: i32 = 4;
const AVATAR_MARGIN_RIGHT: i32 = 42;
const THINKING_MARGIN_BOTTOM: i32 = 24;
const THINKING_MARGIN_RIGHT: i32 = 18;
const CHEVRON_SIZE: i32 = 19;
const CONTENT_BAR_WIDTH: i32 = 6;
const CONTENT_BAR_MARGIN_RIGHT: i32 = 29;

fn draw_chevron(canvas: &Canvas, x: f32, y: f32) {
    let mut path = PathBuilder::new();
    let chevron_size = CHEVRON_SIZE as f32;
    path.move_to((x - chevron_size, y + chevron_size / 2.0));
    path.line_to((x, y - chevron_size / 2.0));
    path.line_to((x + chevron_size, y + chevron_size / 2.0));
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(112, 112, 112));
    paint.set_stroke_width(4.2);
    paint.set_style(PaintStyle::Stroke);
    paint.set_anti_alias(true);
    canvas.draw_path(&path.detach(), &paint);
}

fn deepseek(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut text_paint = Paint::default();
    text_paint.set_color(Color::from_rgb(188, 188, 188));
    let thinking_text =
        Text2Image::from_text("思考中…", 44.0, text_params!(paint = text_paint.clone()));
    let thinking_width = thinking_text.longest_line().ceil() as i32;
    let thinking_height = thinking_text.height().ceil() as i32;
    let content_text = Text2Image::from_text(&texts[0], 44.0, text_params!(paint = text_paint));
    let content_width = content_text.longest_line().ceil() as i32;
    let content_height = content_text.height().ceil() as i32;
    let width_left = AVATAR_WIDTH + AVATAR_MARGIN_RIGHT;
    let width_right = i32::max(
        thinking_width + THINKING_MARGIN_RIGHT + CHEVRON_SIZE * 2,
        CONTENT_BAR_WIDTH + CONTENT_BAR_MARGIN_RIGHT + content_width,
    );
    let width = MARGIN * 2 + width_left + width_right;
    let height_right = thinking_height + THINKING_MARGIN_BOTTOM + content_height;
    let height = MARGIN * 2 + AVATAR_WIDTH.max(height_right);

    let mut surface = new_surface((width, height));
    let canvas = surface.canvas();
    canvas.clear(Color::from_rgb(15, 15, 15));
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(50, 50, 50));
    paint.set_stroke_width(AVATAR_BORDER_WIDTH as f32);
    paint.set_style(PaintStyle::Stroke);
    paint.set_anti_alias(true);
    canvas.draw_circle(
        (MARGIN + AVATAR_WIDTH / 2, MARGIN + AVATAR_WIDTH / 2),
        ((AVATAR_WIDTH - AVATAR_BORDER_WIDTH) / 2) as f32,
        &paint,
    );
    let x = MARGIN + AVATAR_WIDTH + AVATAR_MARGIN_RIGHT;
    thinking_text.draw_on_canvas(canvas, (x, MARGIN));
    draw_chevron(
        canvas,
        (x + thinking_width + THINKING_MARGIN_RIGHT + CHEVRON_SIZE) as f32,
        (MARGIN + thinking_height / 2) as f32,
    );
    let y = MARGIN + thinking_height + THINKING_MARGIN_BOTTOM;
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(80, 80, 80));
    canvas.draw_irect(
        IRect::from_xywh(x, y, CONTENT_BAR_WIDTH, content_height),
        &paint,
    );
    content_text.draw_on_canvas(
        canvas,
        (x + CONTENT_BAR_WIDTH + CONTENT_BAR_MARGIN_RIGHT, y),
    );
    let base = surface.image_snapshot();
    let avatar_size = AVATAR_WIDTH - AVATAR_BORDER_WIDTH * 2;
    let avatar_pos = MARGIN + AVATAR_BORDER_WIDTH;

    make_png_or_gif(images, |images: Vec<Image>| {
        let mut surface = new_surface(base.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&base, (0, 0), None);
        let avatar = images[0]
            .resize_fit((avatar_size, avatar_size), Fit::Cover)
            .circle();
        canvas.draw_image(&avatar, (avatar_pos, avatar_pos), None);
        Ok(surface.image_snapshot())
    })
}

register_meme!(
    "deepseek",
    deepseek,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    keywords = &["思考中"],
    default_texts = &["我操，用户彻底怒了。"],
    date_created = local_date(2026, 6, 9),
    date_modified = local_date(2026, 6, 9),
);
