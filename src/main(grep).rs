use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;

fn visit_dirs(dir: &Path, pattern: &String, cb: &Fn(&DirEntry, &String)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), pattern, cb));
            } else {
                cb(&entry, pattern);
            }
        }
    } else {
        let entry = try!(try!(fs::read_dir(dir)).next().unwrap());
        cb(&entry, pattern);
    }
    Ok(())
}

fn call_back(de: &DirEntry, pt: &String) {
    let mut f = File::open(de.path()).unwrap();
    let mut buf = io::BufReader::new(f);
    for line in io::BufRead::lines(buf) {
        let line = line.unwrap_or("".to_string());
        if line.contains(pt) {
            println!("{}", &line);
        }
    }
}

fn main() {
    let mut arg_iter = args();//参数迭代
    arg_iter.next();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt = arg_iter.next().unwrap_or("./".to_string());
    let pt = Path::new(&pt);
    visit_dirs(&pt, &pattern, &call_back).unwrap();
}
