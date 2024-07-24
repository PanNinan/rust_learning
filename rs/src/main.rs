use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem::size_of;
use std::net::TcpStream;
use std::rc::Rc;
use std::str::Chars;
use std::sync::Arc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Node {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downsteram: Rc<Node>) {
        self.downstream = Some(downsteram);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|x| x.clone())
    }
}

pub fn strtok<'a>(s: &mut &'a str, d: char) -> &'a str {
    if let Some(i) = s.find(d) {
        let prefix = &s[..i];
        let suffix = &s[(i + d.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

macro_rules! show_size {
    (header) => {
        println!("{:<24} {:>8} {:>8} {:>12}", "Type", "T", "Option", "Result");
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:8} {:8} {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
        )
    };
}

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

impl MyWriter<BufWriter<TcpStream>> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        Self {
            writer: BufWriter::new(stream),
        }
    }
}

impl MyWriter<File> {
    pub fn new(path: &str) -> Self {
        Self {
            writer: File::create(path).unwrap(),
        }
    }
}

fn main() {
    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);
    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
    show_size!(Result<String, ()>);

    let mut writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8899");
    writer.write("hello world!").expect("TODO: panic message");
    let mut fileWriter = MyWriter::<File>::new("test.txt");
    fileWriter.write("hello world!").unwrap()
}

fn lifetime1() -> String {
    let name = "Tyr".to_string();
    name[1..].to_string()
}

fn lifetime2(name: String) -> String {
    name[1..].to_string()
}

fn lifetime3(name: &str) -> Chars {
    name.chars()
}

fn test1() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(node4));
    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1: {:?}, node2: {:?}", node1, node2);
}

fn test2() {
    let arr = vec![1];
    std::thread::spawn(move || {
        println!("{:?}", arr);
    });
}

fn test3() {
    let str = String::from("hello");
    let arc = Arc::new(str);
    let arc_clone = arc.clone();
    let handle = std::thread::spawn(move || {
        println!("String from spawned thread: {}", arc_clone);
    });
    println!("String from main thread: {}", arc);
    handle.join().unwrap();
}

fn test4() {
    let s = "hello, world!".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}
