use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use photon_rs::{effects, filters, multiple, PhotonImage, transform};
use photon_rs::native::open_image_from_bytes;
use anyhow::Result;
use photon_rs::effects::oil;

use crate::engine::{Engine, SpecTransform};
use crate::pb::{Contrast, Crop, filter, Filter, Fliph, Flipv, Oil, Resize, resize, Spec, Watermark};
use crate::pb::spec::Data;



lazy_static! {
    // 预先把水印文件加载为静态变量
    // 在编译的时候 include_bytes! 宏会直接把文件读入编译后的二进制
    static ref WATERMARK : PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs {
            match spec.data {
                Some(Data::Crop(ref v)) => self.transform(v),
                Some(Data::Resize(ref v)) => self.transform(v),
                Some(Data::Flipv(ref v)) => self.transform(v),
                Some(Data::Fliph(ref v)) => self.transform(v),
                Some(Data::Contrast(ref v)) => self.transform(v),
                Some(Data::Filter(ref v)) => self.transform(v),
                Some(Data::Watermark(ref v)) => self.transform(v),
                Some(Data::Oil(ref v)) => self.transform(v),
                _ => {}
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let img = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = img
    }
}

impl SpecTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

impl SpecTransform<&Fliph> for Photon {
    fn transform(&mut self, _op: &Fliph) {
        transform::fliph(&mut self.0);
    }
}

impl SpecTransform<&Flipv> for Photon {
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0)
    }
}

impl SpecTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::from_i32(op.filter) {
            Some(filter::Filter::Unspecified) => {}
            Some(f) => filters::filter(&mut self.0, f.to_str().unwrap()),
            _ => {}
        }
    }
}

impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match resize::ResizeType::from_i32(op.rtype).unwrap() {
            resize::ResizeType::Normal => transform::resize(
                &self.0, op.width, op.height, resize::SampleFilter::from_i32(op.filter).unwrap().into(),
            ),
            resize::ResizeType::SeamCarve => transform::seam_carve(&self.0, op.width, op.height),
        };
        self.0 = img
    }
}

impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }
}

impl SpecTransform<&Oil> for Photon {
    fn transform(&mut self, op: &Oil) {
        oil(&mut self.0, op.radius as i32, op.intensity as f64)
    }
}

fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();
    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let image = DynamicImage::ImageRgba8(img_buffer);
    let mut buffer = Vec::with_capacity(32768);
    image.write_to(&mut buffer, format).unwrap();

    buffer
}
