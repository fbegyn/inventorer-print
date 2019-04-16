extern crate inventorer_print;
extern crate structopt;
extern crate regex;

use self::inventorer_print::*;
use std::io;

use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt, Debug)]
#[structopt(name = "printer")]
struct Opt {
    #[structopt(short = "d", long = "database")]
    db: bool,
    #[structopt(short = "b", long = "printer", default_value = "Brother-QL-820NWB")]
    printer: String,
}

fn main(){
    let opt = Opt::from_args();
    if opt.db {
        db_print();
    } else {
        loop {
            manual_print(&opt.printer);
        }
    }
}

fn manual_print(printer: &str){
    println!("Item:");
    let mut item = String::new();
    io::stdin()
        .read_line(&mut item)
        .expect("failed to read from stdio");
    let item = item.trim();
    println!("Location:");
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("failed to read from stdio");
    let location = location.trim();
    println!("Owner:");
    let mut owner = String::new();
    io::stdin()
        .read_line(&mut owner)
        .expect("failed to read from stdio");
    let owner = owner.trim();
    println!("Barcode:");
    let mut barcode = String::new();
    io::stdin()
        .read_line(&mut barcode)
        .expect("failed to read from stdio");
    let barcode = barcode.trim();
    let re = Regex::new(r"[0-9]{12,13}").unwrap();
    if re.is_match(barcode) {
        println!("generating barcode label ...");
        // generate the barcode image
        barcode_to_ean13(barcode, "/tmp/barcode.png");
        render_small_label_barcode(item,  location,  owner, &barcode, "/tmp/barcode.jpg", "/tmp/label.pdf");
    } else {
        println!("generating label ...");
        render_small_label(item,  location,  owner, "/tmp/label.pdf");
    }
    // generate and print the label
    print_barcode(printer,"/tmp/label.pdf");
}

fn db_print(){

}
