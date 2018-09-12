extern crate calamine;

use calamine::{Reader, open_workbook_auto, Xlsx, DataType};

fn main() {
    run();
    let data = vec![vec![1, 2], vec![3, 4], vec![5, 6, 7, 8]];
    let new_data = data.iter().flat_map(|v| v); //Flat_map类型
    println!("new_data :{:?}", new_data);
    for v in new_data {
        println!("v:{:?}", *v);
    }

    (0..5).flat_map(|x| x * 100..x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{}:{}", i, x));
}

fn run() {
    let path = "/home/enziog/Desktop/1.xlsx";
    let mut wb = open_workbook_auto(path).expect("Can't open Excel file");
    let sheets = wb.sheet_names().to_owned();
    for s in sheets {
        println!("found {} formula in '{}'",
                 wb
                     .worksheet_formula(&s)
                     .expect("sheet not found")
                     .expect("error while getting formula")
                     .rows().flat_map(|r| r.iter().filter(|f| !f.is_empty()))
                     .count(),
                 s);
    }
}
