mod abi;

use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
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
impl From<ImageSpec> for String {
    fn from(value: ImageSpec) -> Self {
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
    pub fn to_string(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

impl From<SampleFilter> for SampleFilter {
    fn from(value: SampleFilter) -> Self {
        match value {
            SampleFilter::Undefined => SampleFilter::Nearest,
            SampleFilter::Nearest => SampleFilter::Nearest,
            SampleFilter::Triangle => SampleFilter::Triangle,
            SampleFilter::CatmullRom => SampleFilter::CatmullRom,
            SampleFilter::Gaussian => SampleFilter::Gaussian,
            SampleFilter::Lanczos3 => SampleFilter::Lanczos3,
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
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }
}
