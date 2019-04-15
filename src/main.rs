extern crate barcoders;
extern crate printpdf;

use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::process::Command;

use printpdf::*;
use barcoders::generators;
use barcoders::sym::ean13::*;

fn barcode_to_ean13(code: &str, file: &str){
    let code = EAN13::new(code.to_owned()).unwrap();
    let encoded = code.encode();

    let png = generators::image::Image::png(62);
    let bytes = png.generate(&encoded[..]).unwrap();

    let file = File::create(&Path::new(file)).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

fn print_barcode(file: &str){
    Command::new("lp")
        .arg("-dBrother-QL-820NWB")
        .arg(file)
        .output()
        .expect("Failed to print barcode");
}

fn render_small_label(item: &str, location: &str, team: &str, barcode: &str, image: &str, output: &str){
    let (doc, page, text_layer) = PdfDocument::new("label", Mm(90.0), Mm(31.0), "information");
    // render text layer
    let text_layer = doc.get_page(page).get_layer(text_layer);
    let font = doc.add_external_font(File::open("/usr/share/fonts/TTF/Roboto-Medium.ttf").unwrap()).unwrap();
    text_layer.use_text(String::from("item: ")+item, 12, Mm(3.0),Mm(22.5), &font);
    text_layer.use_text(String::from("location: ")+location, 12, Mm(3.0),Mm(17.5), &font);
    text_layer.use_text(String::from("team: ")+team, 12, Mm(3.0),Mm(12.5), &font);
    text_layer.use_text(String::from("barcode: ")+barcode, 12, Mm(3.0),Mm(7.5), &font);
    // render barcode image
    let image_layer = doc.get_page(page).add_layer("barcode");
    let image_file = File::open(image).unwrap();
    let mut reader = BufReader::new(image_file);
    let image = printpdf::Image::try_from(printpdf::image::png::PNGDecoder::new(&mut reader)).expect("failed to read image");
    println!("{:?}", image);
    image.add_to_layer(image_layer.clone(), Some(Mm(5.0)), Some(Mm(5.0)), None, None, None, Some(10.0));

    doc.save(&mut BufWriter::new(File::create(output).unwrap())).unwrap();
}

fn main(){
    println!("Barcode:");
    let mut barcode = String::new();
    io::stdin()
        .read_line(&mut barcode)
        .expect("failed to read from stdio");
    let barcode = barcode.trim();

    barcode_to_ean13("012345678901", "/tmp/barcode.png");
    render_small_label("schroevendraaier",  "rek 5",  "site&safety", "012345678901", "/tmp/barcode.png", "/tmp/label.pdf");
    print_barcode("/tmp/label.pdf");
}
