//! 特别感谢：O'RLY? 生成器的原作者 nanmu42
//! https://orly.nanmu.me/

use std::array;

use meme_generator_core::error::Error::{self, MemeFeedback};
use meme_generator_utils::builder::{InputImage, MemeOptions};
use meme_generator_utils::config::IMAGES_DIR;
use meme_generator_utils::encoder::{encode_png, make_png_or_gif};
use meme_generator_utils::image::{Fit, ImageExt};
use meme_generator_utils::text::Text2Image;
use meme_generator_utils::text_params;
use meme_generator_utils::tools::{load_image, local_date, new_surface};
use rand::seq::IndexedRandom;
use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::runtime_effect::ChildPtr;
use skia_safe::{
    Color,
    Data,
    FontStyle,
    Image,
    Paint,
    Rect,
    RuntimeEffect,
    SamplingOptions,
    TileMode,
    color_filters,
    image_filters,
};

use crate::color::parse;
use crate::image::{compose_filters, filter_grayscale, flatten_grayscale};
use crate::register_meme;
use crate::text::has_wrap;

const BUILTIN_COLORS: [Color; 17] = [
    Color::from_rgb(97, 0, 94),
    Color::from_rgb(112, 112, 109),
    Color::from_rgb(137, 0, 41),
    Color::from_rgb(196, 0, 14),
    Color::from_rgb(109, 0, 29),
    Color::from_rgb(106, 0, 189),
    Color::from_rgb(241, 0, 0),
    Color::from_rgb(0, 113, 177),
    Color::from_rgb(249, 188, 0),
    Color::from_rgb(44, 0, 119),
    Color::from_rgb(186, 0, 154),
    Color::from_rgb(0, 144, 71),
    Color::from_rgb(0, 157, 158),
    Color::from_rgb(34, 46, 133),
    Color::from_rgb(189, 0, 46),
    Color::from_rgb(0, 157, 26),
    Color::from_rgb(117, 165, 0),
];

fn parse_color(s: &str) -> Option<Color> {
    if let Ok(color_id) = s.parse::<usize>() {
        if color_id == 0 {
            let mut rng = rand::rng();
            return Some(*BUILTIN_COLORS.choose(&mut rng).unwrap());
        } else if color_id <= BUILTIN_COLORS.len() {
            return Some(BUILTIN_COLORS[color_id - 1]);
        }
    }
    parse(s)
}

fn builtin_image(id: Option<i32>) -> Result<Image, Error> {
    let mut rng = rand::rng();
    let dir = IMAGES_DIR.join("idhagnmemes/orly/builtin_images");
    let mut builtin_images = match dir.read_dir() {
        Ok(files) => files
            .flatten()
            .filter_map(|x| {
                if x.metadata().is_ok_and(|x| x.is_file()) {
                    x.file_name()
                        .into_string()
                        .ok()
                        .filter(|x| x.ends_with(".png") || x.ends_with(".jpg"))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        Err(_) => return Err(Error::ImageAssetMissing("内置图片不可用".into())),
    };
    builtin_images.sort();
    let filename = if let Some(id) = id
        && id != 0
    {
        let len = builtin_images.len();
        if len == 0 {
            return Err(Error::ImageAssetMissing("内置图片不可用".into()));
        }
        if id < 0 || (id as usize) > len {
            return Err(Error::MemeFeedback(format!(
                "图片无效，内置图片从 0 到 {}，0 为随机",
                len
            )));
        }
        &builtin_images[id as usize - 1]
    } else {
        builtin_images
            .choose(&mut rng)
            .ok_or_else(|| Error::ImageAssetMissing("内置图片不可用".into()))?
    };
    load_image(format!("idhagnmemes/orly/builtin_images/{}", filename))
}

/// 修改后的 louvre
fn make_sketch(image: &Image, pencil: &Image) -> Result<Image, Error> {
    let lut = array::from_fn(|i| if i > 128 { 0 } else { 255 });
    let smooth = image_filters::blur((0.5, 0.5), None, None, None)
        .ok_or_else(|| Error::MemeFeedback("内部错误：初始化滤镜失败".into()))?;
    let shade = filter_grayscale(
        image,
        compose_filters(vec![
            image_filters::color_filter(
                // 这里用 table 不行，就算是灰度图片
                color_filters::table_argb(None, &lut, &lut, &lut)
                    .ok_or_else(|| Error::MemeFeedback("内部错误：初始化滤镜失败".into()))?,
                None,
                None,
            )
            .ok_or_else(|| Error::MemeFeedback("内部错误：初始化滤镜失败".into()))?,
            smooth.clone(),
            image_filters::arithmetic(
                1.0,
                -1.0,
                0.0,
                1.0,
                false,
                image_filters::image(pencil, None, None, None)
                    .ok_or_else(|| Error::MemeFeedback("内部错误：初始化滤镜失败".into()))?,
                None,
                None,
            )
            .ok_or_else(|| Error::MemeFeedback("内部错误：初始化滤镜失败".into()))?,
        ])
        .ok_or_else(|| Error::MemeFeedback("内部错误：初始化滤镜失败".into()))?,
    )
    .ok_or_else(|| Error::MemeFeedback("内部错误：应用滤镜失败".into()))?;
    let blurred = filter_grayscale(
        image,
        image_filters::blur((1.75, 1.75), TileMode::Clamp, None, None)
            .ok_or_else(|| Error::MemeFeedback("内部错误：初始化滤镜失败".into()))?,
    )
    .ok_or_else(|| Error::MemeFeedback("内部错误：应用滤镜失败".into()))?;
    let sksl = r#"
        uniform shader image;
        uniform shader blurred;
        uniform shader shade;

        float4 main(vec2 coord) {
            float yImage = image.eval(coord).r;
            float yBlurred = blurred.eval(coord).r;
            float yShade = shade.eval(coord).r;
            float yOut = (yImage - yBlurred) * 0.5 + 0.5;
            yOut = step(0.49, yOut);
            yOut = min(yOut, yShade);
            return float4(yOut, yOut, yOut, 1.0);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl, None)
        .map_err(|err| Error::MemeFeedback(format!("内部错误：编译着色器失败: {}", err)))?;
    let shader = effect
        .make_shader(
            Data::new_empty(),
            &[
                ChildPtr::Shader(
                    image
                        .to_shader(None, SamplingOptions::default(), None)
                        .ok_or_else(|| Error::MemeFeedback("内部错误：加载纹理失败".into()))?,
                ),
                ChildPtr::Shader(
                    blurred
                        .to_shader(None, SamplingOptions::default(), None)
                        .ok_or_else(|| Error::MemeFeedback("内部错误：加载纹理失败".into()))?,
                ),
                ChildPtr::Shader(
                    shade
                        .to_shader(None, SamplingOptions::default(), None)
                        .ok_or_else(|| Error::MemeFeedback("内部错误：加载纹理失败".into()))?,
                ),
            ],
            None,
        )
        .ok_or_else(|| Error::MemeFeedback("内部错误：初始化着色器失败".into()))?;
    let mut surface = new_surface(image.dimensions());
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_shader(shader);
    paint.set_image_filter(smooth);
    canvas.draw_rect(Rect::from_size(image.dimensions()), &paint);
    Ok(surface.image_snapshot())
}

#[derive(Clone, Copy)]
enum Style {
    Original,
    Grayscale,
    Sketch,
}

#[derive(MemeOptions)]
struct OrlyOptions {
    /// 页眉
    #[option(short, long)]
    header: Option<String>,

    /// 副标题
    #[option(short, long)]
    subtitle: Option<String>,

    /// 副标题方位
    #[option(short, long, default="rb", choices=["左上", "左下", "右上", "右下", "lt", "lb", "rt", "rb"])]
    position: Option<String>,

    /// 作者
    #[option(short, long)]
    author: Option<String>,

    /// 颜色
    #[option(short, long)]
    color: Option<String>,

    /// 内置图片
    #[option(short, long)]
    builtin_image: Option<i32>,

    /// 外部图片样式
    #[option(long, default = "sketch", choices=["原图", "灰度", "素描", "original", "grayscale", "sketch"])]
    style: Option<String>,
}

fn orly(
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: OrlyOptions,
) -> Result<Vec<u8>, Error> {
    let (title1, title2): (&str, &str) = match texts.len() {
        1 => (&texts[0], ""),
        2 => (&texts[0], &texts[1]),
        _ => unreachable!(),
    };
    let header = &options.header.unwrap_or_default();
    let subtitle = &options.subtitle.unwrap_or_default();
    let author = &options.author.unwrap_or_default();
    if has_wrap(title1)
        || has_wrap(title2)
        || has_wrap(header)
        || has_wrap(subtitle)
        || has_wrap(author)
    {
        return Err(Error::MemeFeedback("内容不能有换行".to_string()));
    }
    let mut rng = rand::rng();
    let color = if let Some(color) = options.color
        && !color.is_empty()
    {
        parse_color(&color).ok_or_else(|| {
            MemeFeedback(format!(
                "颜色无效，内置颜色从 0 到 {}，0 为随机",
                BUILTIN_COLORS.len()
            ))
        })?
    } else {
        *BUILTIN_COLORS.choose(&mut rng).unwrap()
    };
    let style = match options.style.as_deref() {
        Some("原图" | "original") => Style::Original,
        Some("灰度" | "grayscale") => Style::Grayscale,
        Some("素描" | "sketch") => Style::Sketch,
        _ => unreachable!(),
    };

    let medium = FontStyle::new(Weight::MEDIUM, Width::NORMAL, Slant::Upright);
    let heavy = FontStyle::new(Weight::BLACK, Width::NORMAL, Slant::Upright);
    let mut surface = new_surface((1000, 1400));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    if !header.is_empty() {
        let t2i =
            Text2Image::from_text(header.to_string(), 28.0, text_params!(font_style = medium));
        let width = t2i.longest_line();
        if width > 920.0 {
            return Err(Error::TextOverLength(header.to_string()));
        }
        t2i.draw_on_canvas(canvas, (500 - (width / 2.0).round() as i32, 19));
    }
    let mut rect_paint = Paint::default();
    rect_paint.set_color(color);
    canvas.draw_rect(Rect::from_ltrb(40.0, 0.0, 960.0, 19.0), &rect_paint);
    let mut rect_y = 802.0;
    if !subtitle.is_empty() {
        let t2i = Text2Image::from_text(
            subtitle.to_string(),
            39.0,
            text_params!(font_style = medium),
        );
        let width = t2i.longest_line().ceil() as i32;
        let height = t2i.height().ceil();
        if width > 920 {
            return Err(Error::TextOverLength(header.to_string()));
        }
        match options.position.as_deref() {
            Some("左上" | "lt") => {
                t2i.draw_on_canvas(canvas, (40, 801));
                rect_y += height;
            }
            Some("左下" | "lb") => {
                t2i.draw_on_canvas(canvas, (40, 1072));
            }
            Some("右上" | "rt") => {
                t2i.draw_on_canvas(canvas, (959 - width, 801));
                rect_y += height;
            }
            Some("右下" | "rb") => {
                t2i.draw_on_canvas(canvas, (959 - width, 1072));
            }
            _ => return Err(Error::MemeFeedback("内部错误: position 参数无效".into())),
        }
    }
    canvas.draw_rect(
        Rect::from_ltrb(40.0, rect_y, 960.0, rect_y + 270.0),
        &rect_paint,
    );
    let mut title_paint = Paint::default();
    title_paint.set_color(Color::WHITE);
    let title_params = text_params!(
        font_style = FontStyle::bold(),
        paint = title_paint,
        font_families = &["Noto Serif CJK SC"],
    );
    if title2.is_empty() {
        let t2i = Text2Image::from_text(title1.to_string(), 118.0, title_params);
        let width = t2i.longest_line();
        if width > 864.0 {
            return Err(Error::TextOverLength(title1.to_string()));
        }
        t2i.draw_on_canvas(canvas, (68.0, rect_y + 247.0 - t2i.height()));
    } else {
        let t2i = Text2Image::from_text(title1.to_string(), 77.0, title_params.clone());
        let width = t2i.longest_line();
        if width > 864.0 {
            return Err(Error::TextOverLength(title1.to_string()));
        }
        t2i.draw_on_canvas(canvas, (68.0, rect_y + 144.0 - t2i.height()));
        let t2i = Text2Image::from_text(title2.to_string(), 77.0, title_params);
        let width = t2i.longest_line();
        if width > 864.0 {
            return Err(Error::TextOverLength(title2.to_string()));
        }
        t2i.draw_on_canvas(canvas, (68.0, rect_y + 236.0 - t2i.height()));
    }
    let t2i = Text2Image::from_text("O'RLY?".to_string(), 44.0, text_params!(font_style = heavy));
    let orly_width = t2i.longest_line();
    t2i.draw_on_canvas(canvas, (56.0, 1356.0 - t2i.height()));
    if !author.is_empty() {
        let t2i =
            Text2Image::from_text(author.to_string(), 33.0, text_params!(font_style = medium));
        let width = t2i.longest_line();
        if width > 880.0 - orly_width {
            return Err(Error::TextOverLength(author.to_string()));
        }
        t2i.draw_on_canvas(canvas, (944.0 - width, 1353.0 - t2i.height()));
    }
    let base_image = surface.image_snapshot();
    let pencil = load_image("idhagnmemes/orly/sketch.png")?.resize_fit((920, 707), Fit::Cover);

    let make = |image: &Image, style: Style| {
        let mut surface = new_surface(base_image.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&base_image, (0, 0), None);
        let width = image.width() as f32;
        let height = image.height() as f32;
        let ratio = (920.0 / width).min(707.0 / height);
        let width = (width * ratio).round() as i32;
        let height = (height * ratio).round() as i32;
        let image = image.resize_exact((width, height));
        let image = match style {
            Style::Original => image,
            Style::Grayscale => flatten_grayscale(&image)
                .ok_or_else(|| Error::MemeFeedback("内部错误：无法去色图像".into()))?,
            Style::Sketch => make_sketch(
                &flatten_grayscale(&image)
                    .ok_or_else(|| Error::MemeFeedback("内部错误：无法去色图像".into()))?,
                &pencil,
            )?,
        };
        canvas.draw_image(&image, (960 - image.width(), 802 - image.height()), None);
        Ok(surface.image_snapshot())
    };

    match (images.is_empty(), options.builtin_image) {
        (false, Some(_)) => Err(Error::MemeFeedback("不能同时使用内置图片和外部图片".into())),
        (false, None) => make_png_or_gif(images, |images| make(&images[0], style)),
        (true, builtin) => Ok(encode_png(make(
            &builtin_image(builtin)?,
            Style::Original,
        )?)?),
    }
}

register_meme!(
    "orly",
    orly,
    min_texts = 1,
    max_texts = 2,
    min_images = 0,
    max_images = 1,
    keywords = &["orly", "动物书"],
    default_texts = &["表情包制作", "从入门到入土"],
    date_created = local_date(2022, 7, 28),
    date_modified = local_date(2026, 6, 1),
);
