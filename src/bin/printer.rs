extern crate inventorer_print;

use self::inventorer_print::*;
//use std::io;

fn main(){
    //println!("Barcode:");
    //let mut barcode = String::new();
    //io::stdin()
    //    .read_line(&mut barcode)
    //    .expect("failed to read from stdio");
    //let barcode = barcode.trim();

    //barcode_to_ean13(barcode, "/tmp/barcode.png");
    barcode_to_ean13("012345678901", "/tmp/barcode.jpg");
    render_small_label("schroevendraaier",  "rek 5",  "site&safety", "012345678901", "/tmp/barcode.jpg", "/tmp/label.pdf");
    //print_barcode("Brother-QL-820NWB","/tmp/label.pdf");
}
