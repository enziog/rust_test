extern crate actix_web;
extern crate listenfd;

use actix_web::{server, App, HttpRequest, Responder};
use listenfd::ListenFd;

fn index(_req: &HttpRequest) -> impl Responder {
    "你好!This is my first 不支持中文的 actix-web based app"
}

fn main() {
    let mut s = String::from("你好 EnzioG");
    let word = first_word(&s);
    println!("first word counts:{}", word);

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}", item);
        if item == b' ' {
            return i - 1;
        }
    }
    s.len()
}