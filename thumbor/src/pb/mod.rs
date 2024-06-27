mod abi;

use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;
use prost::Message;
// 声明 abi.rs
use crate::pb::resize::SampleFilter;
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// 让ImageSpec可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(value: &ImageSpec) -> Self {
        let data = value.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

// 让ImageSpec 可以通过一个字符串创建 如 s.parse().unwrap()
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// 辅助函数，photon_rs 相应的方法里需要字符串
impl filter::Filter {
    pub fn to_str(self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
            filter::Filter::Radio => Some("radio"),
            filter::Filter::Twenties => Some("twenties"),
        }
    }
}

impl From<SampleFilter> for SamplingFilter {
    fn from(value: SampleFilter) -> Self {
        match value {
            SampleFilter::Undefined => SamplingFilter::Nearest,
            SampleFilter::Nearest => SamplingFilter::Nearest,
            SampleFilter::Triangle => SamplingFilter::Triangle,
            SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            SampleFilter::Gaussian => SamplingFilter::Gaussian,
            SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
    
    pub fn new_oil(x: u32, y: f64) -> Self {
        Self {
            data: Some(spec::Data::Oil(Oil {
                radius: x,
                intensity: y as f32,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn encode_spec_could_be_decode() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
