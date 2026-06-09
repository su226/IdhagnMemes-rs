use meme_generator_core::error::Error;
use meme_generator_utils::builder::InputImage;
use meme_generator_utils::encoder::make_png_or_gif;
use meme_generator_utils::tools::{local_date, new_surface};
use skia_safe::runtime_effect::ChildPtr;
use skia_safe::{Data, IRect, Image, Paint, RuntimeEffect, SamplingOptions, TileMode};

use crate::image::set_uniform_f32;
use crate::options::NoOptions;
use crate::register_meme;

fn make_dots(image: &Image) -> Image {
    let sksl = r#"
        uniform shader image;
        uniform float dotSize;
        uniform float dotOffset;
        uniform float dotScale;

        float4 main(float2 coord) {
            float offset = mod(floor(coord.y / dotSize) * dotOffset, dotSize);
            float halfDotSize = dotSize * 0.5;
            float centerX = floor((coord.x - offset) / dotSize) * dotSize + halfDotSize;
            float centerY = floor(coord.y / dotSize) * dotSize + halfDotSize;
            float posX = mod((coord.x - offset), dotSize) - halfDotSize;
            float posY = mod(coord.y, dotSize) - halfDotSize;
            float dotRadius = halfDotSize * dotScale;
            float alpha = clamp(dotRadius + 0.5 - sqrt(posX * posX + posY * posY), 0, 1);
            return image.eval(float2(centerX, centerY)) * alpha;
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl, None);
    let effect = match effect {
        Ok(effect) => effect,
        Err(err) => panic!("{}", err),
    };
    let mut uniforms = vec![0; effect.uniform_size()];
    set_uniform_f32(&mut uniforms, effect.find_uniform("dotSize").unwrap(), 10.0).unwrap();
    set_uniform_f32(
        &mut uniforms,
        effect.find_uniform("dotOffset").unwrap(),
        5.0,
    )
    .unwrap();
    set_uniform_f32(&mut uniforms, effect.find_uniform("dotScale").unwrap(), 0.8).unwrap();
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

fn dots(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    make_png_or_gif(images, |images: Vec<Image>| Ok(make_dots(&images[0])))
}

register_meme!(
    "dots",
    dots,
    min_images = 1,
    max_images = 1,
    keywords = &["圆点"],
    date_created = local_date(2026, 6, 6),
    date_modified = local_date(2026, 6, 6),
);
