use std::array;

use meme_generator_core::error::Error;
use meme_generator_utils::builder::{InputImage, MemeOptions};
use meme_generator_utils::encoder::make_png_or_gif;
use meme_generator_utils::image::{Fit, ImageExt};
use meme_generator_utils::tools::{load_image, local_date, new_surface};
use skia_safe::gradient::{Colors, Gradient, Interpolation};
use skia_safe::runtime_effect::{ChildPtr, Uniform};
use skia_safe::shaders::linear_gradient;
use skia_safe::{
    AlphaType,
    Color,
    Color4f,
    ColorType,
    Data,
    IPoint,
    ISize,
    Image,
    ImageFilter,
    ImageInfo,
    Paint,
    Rect,
    RuntimeEffect,
    SamplingOptions,
    Shader,
    Surface,
    TileMode,
    color_filters,
    image_filters,
    surfaces,
};

use crate::register_meme;

fn make_gray_surface(width: i32, height: i32) -> Option<Surface> {
    surfaces::raster(
        &ImageInfo::new(
            ISize::new(width, height),
            ColorType::Gray8,
            AlphaType::Opaque,
            None,
        ),
        None,
        None,
    )
}

fn grayscale(image: &Image) -> Option<Image> {
    let mut surface = make_gray_surface(image.width(), image.height())?;
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(image, (0, 0), None);
    Some(surface.image_snapshot())
}

fn filter_gray(image: &Image, filter: ImageFilter) -> Option<Image> {
    let mut surface = make_gray_surface(image.width(), image.height())?;
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_image_filter(Some(filter));
    canvas.draw_image(image, (0, 0), Some(&paint));
    Some(surface.image_snapshot())
}

fn make_gradient(dimensions: ISize) -> Option<Shader> {
    linear_gradient(
        ((0, 0), (dimensions.width, dimensions.height)),
        &Gradient::new(
            Colors::new(
                &[
                    Color4f::new(0.9843137, 0.7294118, 0.1882353, 1.0),
                    Color4f::new(0.9882353, 0.44705883, 0.20784314, 1.0),
                    Color4f::new(0.9882353, 0.20784314, 0.30588236, 1.0),
                    Color4f::new(0.8117647, 0.21176471, 0.8745098, 1.0),
                    Color4f::new(0.21568628, 0.70980394, 0.8509804, 1.0),
                    Color4f::new(0.24313726, 0.7137255, 0.85490197, 1.0),
                ],
                Some(&[0.0, 0.4, 0.6, 0.7, 0.8, 1.0]),
                TileMode::Clamp,
                None,
            ),
            Interpolation::default(),
        ),
        None,
    )
}

fn get_average_kernel(size: usize) -> (Vec<f32>, ISize, IPoint) {
    let size_square = size * size;
    let kernel = vec![1.0 / size_square as f32; size_square];
    let kernel_size = ISize::new(size as i32, size as i32);
    let kernel_offset = IPoint::new(size as i32 / 2, size as i32 / 2);
    (kernel, kernel_size, kernel_offset)
}

fn get_kernel(style: &str) -> Option<(Vec<f32>, ISize, IPoint)> {
    match style {
        "thin" => Some(get_average_kernel(5)),
        "normal" => Some(get_average_kernel(7)),
        "semibold" => Some(get_average_kernel(9)),
        "bold" => Some(get_average_kernel(11)),
        "black" => Some(get_average_kernel(13)),
        "emboss" => Some((
            vec![
                1.0, 1.0, 1.0, //
                1.0, 1.0, -1.0, //
                -1.0, -1.0, -1.0,
            ],
            ISize::new(3, 3),
            IPoint::new(1, 1),
        )),
        _ => None,
    }
}

fn compose_multi(filters: Vec<ImageFilter>) -> Option<ImageFilter> {
    let mut it = filters.into_iter();
    let mut filter = it.next()?;
    for next_filter in it {
        filter = image_filters::compose(next_filter, filter)?;
    }
    Some(filter)
}

fn set_uniform_f32(uniforms: &mut [u8], uniform: &Uniform, value: f32) -> Result<(), Error> {
    let uniform_offset = uniform.offset();
    let uniform_size = uniform.size_in_bytes();
    let data = value.to_ne_bytes();
    let data_size = data.len();
    if uniform_size != data_size {
        return Err(error(&format!("Uniform 大小应该为 {}", data_size)));
    }
    uniforms[uniform_offset..(uniform_size + uniform_offset)]
        .copy_from_slice(&data[..uniform_size]);
    Ok(())
}

#[inline(always)]
fn error(reason: &str) -> Error {
    Error::MemeFeedback(format!("内部错误: {}", reason))
}

const SHADE_LIGHT: u8 = 80;
const LIGHT_CUT: u8 = 128;

fn make_louvre(
    image: &Image,
    pencil: &Image,
    gradient: Shader,
    style: &str,
    dark_cut: i32,
    shade_limit: i32,
    denoise: bool,
) -> Result<Image, Error> {
    let lut = array::from_fn(|i| if i > shade_limit as usize { 0 } else { 255 });
    let shade_light = (SHADE_LIGHT as f32) / 255.0;
    let shade = filter_gray(
        image,
        compose_multi(vec![
            image_filters::color_filter(
                // 这里用 table 不行，就算是灰度图片
                color_filters::table_argb(None, &lut, &lut, &lut)
                    .ok_or_else(|| error("初始化滤镜失败"))?,
                None,
                None,
            )
            .ok_or_else(|| error("初始化滤镜失败"))?,
            image_filters::blur((1.0, 1.0), None, None, None).unwrap(),
            image_filters::arithmetic(
                -shade_light,
                shade_light,
                0.0,
                0.0,
                false,
                image_filters::image(pencil, None, None, None)
                    .ok_or_else(|| error("初始化滤镜失败"))?,
                None,
                None,
            )
            .ok_or_else(|| error("初始化滤镜失败"))?,
        ])
        .ok_or_else(|| error("初始化滤镜失败"))?,
    )
    .ok_or_else(|| error("应用滤镜失败"))?;
    let image = if denoise {
        &filter_gray(
            image,
            image_filters::matrix_convolution(
                ISize::new(3, 3),
                &[1.0 / 9.0; 9],
                1.0,
                0.0,
                IPoint::new(1, 1),
                TileMode::Clamp,
                false,
                None,
                None,
            )
            .ok_or_else(|| error("初始化卷积失败"))?,
        )
        .ok_or_else(|| error("应用滤镜失败"))?
    } else {
        image
    };
    let (kernel, kernel_size, kernel_offset) =
        get_kernel(style).ok_or_else(|| error("风格无效"))?;
    let convolved = filter_gray(
        image,
        image_filters::matrix_convolution(
            kernel_size,
            &kernel,
            1.0,
            0.0,
            kernel_offset,
            TileMode::Clamp,
            false,
            None,
            None,
        )
        .ok_or_else(|| error("初始化卷积失败"))?,
    )
    .ok_or_else(|| error("应用滤镜失败"))?;
    let sksl = r#"
        uniform shader image;
        uniform shader convolved;
        uniform shader shade;
        uniform shader gradient;
        uniform float darkCut;
        uniform float scale;

        float4 main(vec2 coord) {
            float yImage = image.eval(coord).r;
            float yConvolved = convolved.eval(coord).r;
            float yShade = shade.eval(coord).r;
            float3 rgbGradient = gradient.eval(coord).rgb;
            float aGradient = yImage - yConvolved + 0.5;
            aGradient = clamp((aGradient - darkCut) * scale, 0.0, 1.0);
            aGradient = max(1.0 - aGradient, yShade);
            return float4(rgbGradient * aGradient + 1.0 * (1.0 - aGradient), 1.0);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl, None)
        .map_err(|err| error(&format!("编译着色器失败: {}", err)))?;
    let mut uniforms = vec![0; effect.uniform_size()];
    set_uniform_f32(
        &mut uniforms,
        effect
            .find_uniform("darkCut")
            .ok_or_else(|| error("Uniform 不存在"))?,
        dark_cut as f32 / 255.0,
    )?;
    set_uniform_f32(
        &mut uniforms,
        effect
            .find_uniform("scale")
            .ok_or_else(|| error("Uniform 不存在"))?,
        255.0 / (255.0 - (LIGHT_CUT as f32) - (dark_cut as f32)),
    )?;
    let shader = effect
        .make_shader(
            Data::new_copy(&uniforms),
            &[
                ChildPtr::Shader(
                    image
                        .to_shader(None, SamplingOptions::default(), None)
                        .ok_or_else(|| error("加载纹理失败"))?,
                ),
                ChildPtr::Shader(
                    convolved
                        .to_shader(None, SamplingOptions::default(), None)
                        .ok_or_else(|| error("加载纹理失败"))?,
                ),
                ChildPtr::Shader(
                    shade
                        .to_shader(None, SamplingOptions::default(), None)
                        .ok_or_else(|| error("加载纹理失败"))?,
                ),
                ChildPtr::Shader(gradient),
            ],
            None,
        )
        .ok_or_else(|| error("初始化着色器失败"))?;
    let mut surface = new_surface(image.dimensions());
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_shader(shader);
    canvas.draw_rect(Rect::from_size(image.dimensions()), &paint);
    Ok(surface.image_snapshot())
}

#[derive(MemeOptions)]
struct LouvreOptions {
    /// 线条风格
    #[option(long, default="normal", choices=["thin", "normal", "semibold", "bold", "black", "emboss"])]
    style: Option<String>,

    /// 边缘强度
    #[option(long, default = 118, minimum = 80, maximum = 126)]
    edge: Option<i32>,

    /// 暗部强度
    #[option(long, default = 108, minimum = 20, maximum = 200)]
    shade: Option<i32>,

    /// 降噪
    #[option(long, default = true)]
    denoise: Option<bool>,
}

fn louvre(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: LouvreOptions,
) -> Result<Vec<u8>, Error> {
    let dimensions = images[0].image.dimensions();
    let pencil =
        grayscale(&load_image("idhagnmemes/louvre/0.jpg")?.resize_fit(dimensions, Fit::Cover))
            .ok_or_else(|| error("无法去色素材"))?;
    let gradient = make_gradient(dimensions).ok_or_else(|| error("创建渐变着色器失败"))?;

    make_png_or_gif(images, |images| {
        make_louvre(
            &grayscale(&images[0]).ok_or_else(|| error("无法去色图片"))?,
            &pencil,
            gradient.clone(),
            options.style.as_ref().ok_or_else(|| error("参数不存在"))?,
            options.edge.ok_or_else(|| error("参数不存在"))?,
            options.shade.ok_or_else(|| error("参数不存在"))?,
            options.denoise.ok_or_else(|| error("参数不存在"))?,
        )
    })
}

register_meme!(
    "louvre",
    louvre,
    min_images = 1,
    max_images = 1,
    keywords = &["卢浮宫"],
    date_created = local_date(2022, 8, 22),
    date_modified = local_date(2026, 5, 24),
);
