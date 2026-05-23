use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::GifEncoder;
use meme_generator_utils::image::{Fit, ImageExt};
use meme_generator_utils::tools::{load_image, local_date, new_surface};
use rand::RngExt;
use skia_safe::{Color, ISize, Image, Paint, Rect, Surface, color_filters};

use crate::options::NoOptions;
use crate::register_meme;

const AVATAR_BOX_IN_PRICE: Rect = Rect {
    left: 225.0,
    top: 66.0,
    right: 305.0,
    bottom: 146.0,
};
const SLIDE_FRAMES: i32 = 3;
const SLIDE_DURATION: f32 = 0.15;
const AVATAR_DURATION: f32 = 0.15;
const SCALE_FRAMES: i32 = 3;
const SCALE_DURATION: f32 = 0.15;
const PRICE_FRAMES: i32 = 5;
const PRICE_DURATION: f32 = 0.25;
const PRICE_LEFT: i32 = 24;
const PRICE_TOP: i32 = 93;

fn make_price_surface(bg: &Image, fg: &Image) -> Surface {
    let mut surface = new_surface(bg.dimensions());
    let canvas = surface.canvas();
    let mut rng = rand::rng();
    canvas.draw_image(bg, (0, 0), None);
    canvas.draw_image(
        fg,
        (
            PRICE_LEFT + rng.random_range(-10..=10),
            PRICE_TOP + rng.random_range(-10..=10),
        ),
        None,
    );
    surface
}

fn lerp(a: f32, b: f32, r: f32) -> f32 {
    a * (1.0 - r) + b * r
}

fn from_white(ratio: f32) -> [f32; 20] {
    [
        ratio,
        0.0,
        0.0,
        0.0,
        1.0 - ratio,
        //
        0.0,
        ratio,
        0.0,
        0.0,
        1.0 - ratio,
        //
        0.0,
        0.0,
        ratio,
        0.0,
        1.0 - ratio,
        //
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

fn indihome(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let price_bg_im = load_image("idhagnmemes/indihome/0.png")?;
    let price_fg_im = load_image("idhagnmemes/indihome/1.png")?;
    let ISize { width, height } = price_bg_im.dimensions();
    if width != height {
        return Err(Error::MemeFeedback("素材可能已损坏".to_string()));
    }
    let image = images[0]
        .image
        .resize_fit((width, height), Fit::Cover)
        .circle();
    let mut encoder = GifEncoder::new();
    for i in 0..SLIDE_FRAMES {
        let x = lerp(width as f32, 0.0, (i as f32) / (SLIDE_FRAMES as f32)) as i32;
        let mut surface = new_surface((width, height));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&image, (x, 0), None);
        encoder.add_frame(
            surface.image_snapshot(),
            SLIDE_DURATION / (SLIDE_FRAMES as f32),
        )?;
    }
    encoder.add_frame(image.with_background(Color::WHITE), AVATAR_DURATION)?;
    for i in 0..SCALE_FRAMES {
        let ratio = (i as f32 + 1.0) / (SCALE_FRAMES as f32 + 1.0);
        let price_im = make_price_surface(&price_bg_im, &price_fg_im).image_snapshot();
        let mut surface = new_surface(price_im.dimensions());
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_color_filter(color_filters::matrix_row_major(&from_white(ratio), None));
        canvas.draw_image(price_im, (0, 0), Some(&paint));
        canvas.draw_image_rect(
            &image,
            None,
            Rect::from_ltrb(
                lerp(0.0, AVATAR_BOX_IN_PRICE.left, ratio),
                lerp(0.0, AVATAR_BOX_IN_PRICE.top, ratio),
                lerp(width as f32, AVATAR_BOX_IN_PRICE.right, ratio),
                lerp(height as f32, AVATAR_BOX_IN_PRICE.bottom, ratio),
            ),
            &Paint::default(),
        );
        encoder.add_frame(
            surface.image_snapshot(),
            SCALE_DURATION / (SCALE_FRAMES as f32),
        )?;
    }
    for _ in 0..PRICE_FRAMES {
        let mut surface = make_price_surface(&price_bg_im, &price_fg_im);
        let canvas = surface.canvas();
        canvas.draw_image_rect(&image, None, AVATAR_BOX_IN_PRICE, &Paint::default());
        encoder.add_frame(
            surface.image_snapshot(),
            PRICE_DURATION / (PRICE_FRAMES as f32),
        )?;
    }
    encoder.finish()
}

register_meme!(
    "indihome",
    indihome,
    min_images = 1,
    max_images = 1,
    keywords = &["inidhome", "印尼宽带"],
    date_created = local_date(2022, 2, 14),
    date_modified = local_date(2026, 5, 23),
);
