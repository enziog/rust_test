extern crate calamine;
extern crate tensorflow;

use std::io;
use std::string;
use calamine::{Reader, open_workbook_auto, Xlsx, DataType, open_workbook};

fn main() {
    println!("Hello, world!");
    /*
        let mut teststr = String::new();
        io::stdin().read_line(&mut teststr)
            .expect("failed to read teststr");
        if teststr == String::from("1\n") {
            println!("exit");
        } else {
            println!("go on");
        }
        println!("teststr is: {}", teststr);
        println!("compared string is: {}", &String::from("1"));
    */
    let mut excel: Xlsx<_> = open_workbook("/home/enziog/Rust/rust_test/src/17-0003II-1 材料清单.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Recovered_Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }


    // opens a new workbook
    let path = "/home/enziog/Rust/rust_test/src/17-0003II-1 材料清单.xlsx"; // we do not know the file type
    let mut workbook = open_workbook_auto(path).expect("Cannot open file");

    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range("Recovered_Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!("Found {} cells in 'Sheet1', including {} non empty cells",
                 total_cells, non_empty_cells);
        // alternatively, we can manually filter rows
        assert_eq!(non_empty_cells, range.rows()
            .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty)).count());
    }

    // Check if the workbook has a vba project
    if let Some(Ok(mut vba)) = workbook.vba_project() {
        let vba = vba.to_mut();
        let module1 = vba.get_module("Module 1").unwrap();
        println!("Module 1 code:");
        println!("{}", module1);
        for r in vba.get_references() {
            if r.is_missing() {
                println!("Reference {} is broken or not accessible", r.name);
            }
        }
    }

    // You can also get defined names definition (string representation only)
    for name in workbook.defined_names() {
        println!("name: {}, formula: {}", name.0, name.1);
    }

    // Now get all formula!
    let sheets = workbook.sheet_names().to_owned();
    for s in sheets {
        println!("found {} formula in '{}'",
                 workbook
                     .worksheet_formula(&s)
                     .expect("sheet not found")
                     .expect("error while getting formula")
                     .rows().flat_map(|r| r.iter().filter(|f| !f.is_empty()))
                     .count(),
                 s);
    }
}
