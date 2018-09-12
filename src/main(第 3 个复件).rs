use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;
use std::fmt;


struct Complex(f64, f64);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.1 < 0.0 {
            write!(f, "Display: {}{}i", self.0, self.1)
        } else {
            write!(f, "Display: {}+{}i", self.0, self.1)
        }
    }
}

fn main() {
    let test = Complex(0.5, -0.8);
    println!("{}", test);

    let x = 1;
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c),
        _ => println!("{}", x)
    }

    //println!("x: {}", x);


    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
    println!("{:?}", (5..10).enumerate());

    let mut arg_iter = args();//参数迭代
    println!("{:?}", arg_iter.next());
    arg_iter.next();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt = arg_iter.next().unwrap_or("./".to_string());
    let pt = Path::new(&pt);
}
