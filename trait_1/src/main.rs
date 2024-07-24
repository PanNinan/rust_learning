use std::io::Write;
use std::sync::{Arc, Mutex};

use buf_builder::BufBuilder;

use crate::developer::Developer;
use crate::str_parser::Parse;

mod buf_builder;
mod developer;
mod str_parser;

fn main() {
    let mut buf_builder = BufBuilder::new();
    buf_builder.write_all(b"hello world").unwrap();
    println!(
        "buf p: {:p}, buf[0]p: {:p}, buf[1]p: {:p}, {:?}",
        &buf_builder.buf, &buf_builder.buf[0], &buf_builder.buf[1], buf_builder.buf
    );

    println!("result: {:?}", u8::parse("122121.23123weqwe1233abc"));

    let d1 = Developer::new("jackson");
    let d2: Developer = Default::default();
    let d3 = Developer::default();

    println!("d1 : {}, d2: {}, d3: {:?}", d1, d2, d3);

    let shared = Arc::new(Mutex::new(1));
    let mut g = shared.lock().unwrap();
}
