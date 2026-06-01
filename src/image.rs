use meme_generator_core::error::Error;
use meme_generator_utils::tools::new_surface;
use skia_safe::runtime_effect::Uniform;
use skia_safe::{
    AlphaType,
    Color,
    ColorType,
    ISize,
    Image,
    ImageFilter,
    ImageInfo,
    Paint,
    Surface,
    image_filters,
    surfaces,
};

pub fn new_grayscale_surface(dimensions: impl Into<ISize>) -> Option<Surface> {
    surfaces::raster(
        &ImageInfo::new(dimensions, ColorType::Gray8, AlphaType::Opaque, None),
        None,
        None,
    )
}

pub fn flatten(image: &Image) -> Image {
    let mut surface = new_surface(image.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(image, (0, 0), None);
    surface.image_snapshot()
}

pub fn flatten_grayscale(image: &Image) -> Option<Image> {
    let mut surface = new_grayscale_surface(image.dimensions())?;
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(image, (0, 0), None);
    Some(surface.image_snapshot())
}

pub fn filter_grayscale(image: &Image, filter: ImageFilter) -> Option<Image> {
    let mut surface = new_grayscale_surface(image.dimensions())?;
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_image_filter(Some(filter));
    canvas.draw_image(image, (0, 0), Some(&paint));
    Some(surface.image_snapshot())
}

pub fn compose_filters(filters: Vec<ImageFilter>) -> Option<ImageFilter> {
    let mut it = filters.into_iter();
    let mut filter = it.next()?;
    for next_filter in it {
        filter = image_filters::compose(next_filter, filter)?;
    }
    Some(filter)
}

pub fn set_uniform_f32(uniforms: &mut [u8], uniform: &Uniform, value: f32) -> Result<(), Error> {
    let uniform_offset = uniform.offset();
    let uniform_size = uniform.size_in_bytes();
    let data = value.to_ne_bytes();
    let data_size = data.len();
    if uniform_size != data_size {
        return Err(Error::MemeFeedback(format!(
            "Uniform 大小应该为 {}",
            data_size
        )));
    }
    uniforms[uniform_offset..(uniform_size + uniform_offset)]
        .copy_from_slice(&data[..uniform_size]);
    Ok(())
}
