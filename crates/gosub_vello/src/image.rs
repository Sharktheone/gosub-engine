use gosub_render_backend::{Image as TImage, FP};
use image::DynamicImage;
use vello::peniko::Image as VelloImage;

pub struct Image(pub(crate) VelloImage);

impl From<VelloImage> for Image {
    fn from(image: VelloImage) -> Self {
        Image(image)
    }
}

impl TImage for Image {
    fn new(size: (FP, FP), data: Vec<u8>) -> Self {
        todo!()
    }

    fn from_img(img: &DynamicImage) -> Self {
        todo!()
    }
}
