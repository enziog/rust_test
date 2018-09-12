extern crate glob;

//extern crate calamine;
//use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};
use std::fs;
use std::string;
use std::path::Path;

/*
fn findall(ref dir) {
    match fs::read_dir(dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            let pathR = path.unwrap();
            if pathR.is_dir() {
                findall(&pathR);
            } else {

            println!("> {:?}", path.unwrap().path());
            }
        }
    }
}
*/

fn main() {
    println!("Hello, world!");
    /*
        let dir = Path::new("/usr/bin");
    //    findall(&dir);
        match fs::read_dir(dir) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) =>
    //            for path in paths {
    //                let pathR = path.unwrap();
    //                if pathR.is_dir() {
    //                    findall(&pathR);
    //                } else {
                    println!("> {:?}", paths.unwrap().path()),
    //                }
    //            }
        }
    */
    let path = Path::new("/home/enziog");
    println!("{:?}", path.to_str());

    use glob::glob;

    let mut count = 0;

    for entry in glob("/home/enziog/**/Pic*.jpg").unwrap() {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                count += 1;
            }
            Err(e) => println!("{:?}", e),
        }
    }
    println!("No. of paths found is: {}", count);

//    findall(&dir);

    let one_to_ten = (0..10).collect::<Vec<i32>>();
    for num in one_to_ten {
        println!("{}", num);
    }
}
