use std::process::*;
use std::env::args;

fn main() {
    let mut arg_iter = args();
    println!("{:?}", arg_iter);
    arg_iter.next().unwrap();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    println!("{:?}", pattern);
    let pt = arg_iter.next().unwrap_or("./".to_string());
    let output = Command::new("/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .stdout(Stdio::piped())
        .spawn().unwrap();
    std::thread::sleep_ms(1000);
    println!("计算要花一点点时间");
    let out = output.wait_with_output().unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);
    for line in out_str.split("\n") {
        println!("{}", line);
    }
}
