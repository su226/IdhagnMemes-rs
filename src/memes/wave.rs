use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::make_png_or_gif;
use meme_generator_utils::tools::{local_date, new_surface};
use skia_safe::runtime_effect::ChildPtr;
use skia_safe::{Data, IRect, Image, Paint, RuntimeEffect, SamplingOptions, TileMode};

use crate::image::set_uniform_f32;
use crate::options::NoOptions;
use crate::register_meme;

const MIN_PIXELS: i32 = 20;
const MAX_ROWS: i32 = 100;

fn make_wave(image: &Image) -> Image {
    let sksl = r#"
        uniform shader image;
        uniform float chunkSizeX;
        uniform float chunkSizeY;
        uniform float ratioExp;
        uniform float waveSizeMul;

        float rgba2y(float4 rgba) {
            return 0.299 * rgba.r + 0.587 * rgba.g + 0.114 * rgba.b;
        }

        float lerp(float a, float b, float r) {
            return a * (1 - r) + b * r;
        }

        float4 main(float2 coord) {
            float waveX = mod(coord.x, chunkSizeX);
            float waveY = mod(coord.y, chunkSizeY);
            float halfChunkSizeX = chunkSizeX * 0.5;
            float halfChunkSizeY = chunkSizeY * 0.5;
            float colorX = floor(coord.x / chunkSizeX) * chunkSizeX + halfChunkSizeX;
            float colorY = floor(coord.y / chunkSizeY) * chunkSizeY + halfChunkSizeY;
            float4 color = image.eval(float2(colorX, colorY));
            float r;
            float g;
            float b;
            float a;
            float waveSize = halfChunkSizeY * waveSizeMul;
            if (waveX > halfChunkSizeX) {
                float4 color2 = image.eval(float2(colorX + chunkSizeX, colorY));
                float ratio = (waveX - halfChunkSizeX) / chunkSizeX;
                ratio = pow(ratio * 2, ratioExp);
                waveSize *= lerp(rgba2y(color), rgba2y(color2), ratio);
                r = lerp(color.r, color2.r, ratio);
                g = lerp(color.g, color2.g, ratio);
                b = lerp(color.b, color2.b, ratio);
                a = lerp(color.a, color2.a, ratio);
            } else {
                float4 color2 = image.eval(float2(colorX - chunkSizeX, colorY));
                float ratio = (waveX + halfChunkSizeX) / chunkSizeX;
                ratio = 1 - pow(1 - ratio * 2, ratioExp);
                waveSize *= lerp(rgba2y(color2), rgba2y(color), ratio);
                r = lerp(color2.r, color.r, ratio);
                g = lerp(color2.g, color.g, ratio);
                b = lerp(color2.b, color.b, ratio);
                a = lerp(color2.a, color.a, ratio);
            }
            float ratio = clamp((waveY + 0.5) - (halfChunkSizeY - waveSize), 0, 1) * clamp((halfChunkSizeY + waveSize) - (waveY - 0.5), 0, 1);
            return float4(r, g, b, a) * ratio;
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl, None);
    let effect = match effect {
        Ok(effect) => effect,
        Err(err) => panic!("{}", err),
    };
    let mut uniforms = vec![0; effect.uniform_size()];
    let chunk_size = (image.height() as f32 / MAX_ROWS as f32).max(MIN_PIXELS as f32);
    set_uniform_f32(
        &mut uniforms,
        effect.find_uniform("chunkSizeX").unwrap(),
        chunk_size,
    )
    .unwrap();
    set_uniform_f32(
        &mut uniforms,
        effect.find_uniform("chunkSizeY").unwrap(),
        chunk_size,
    )
    .unwrap();
    set_uniform_f32(
        &mut uniforms,
        effect.find_uniform("ratioExp").unwrap(),
        2.0,
    )
    .unwrap();
    set_uniform_f32(
        &mut uniforms,
        effect.find_uniform("waveSizeMul").unwrap(),
        0.8,
    )
    .unwrap();
    let children = image
        .to_shader(
            (TileMode::Clamp, TileMode::Clamp),
            SamplingOptions::default(),
            None,
        )
        .unwrap();
    let shader = effect
        .make_shader(
            Data::new_copy(&uniforms),
            &[ChildPtr::Shader(children)],
            None,
        )
        .unwrap();
    let mut surface = new_surface(image.dimensions());
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_shader(shader);
    canvas.draw_irect(IRect::from_size(image.dimensions()), &paint);
    surface.image_snapshot()
}

fn wave(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    make_png_or_gif(images, |images: Vec<Image>| Ok(make_wave(&images[0])))
}

register_meme!(
    "wave1",
    wave,
    min_images = 1,
    max_images = 1,
    keywords = &["波浪"],
    date_created = local_date(2026, 6, 5),
    date_modified = local_date(2026, 6, 5),
);
