extern crate barcoders;
extern crate printpdf;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::process::Command;

use printpdf::*;
use barcoders::generators;
use barcoders::sym::ean13::*;

pub fn barcode_to_ean13(code: &str, file: &str) -> std::vec::Vec<u8> {
    let code = EAN13::new(code.to_owned()).expect("failed to create barcode from &str");
    let encoded = code.encode();
    let barcode = generators::image::Image::jpeg(62);
    let bytes = barcode.generate(&encoded[..]).expect("failed to parse barcode into bytes");
    let file = File::create(&Path::new(file)).expect("failed to open file");
    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).expect("failed to write bytes to file");
    return bytes;
}

pub fn print_barcode(printer: &str, file: &str){
    Command::new("lp")
        .arg("-d")
        .arg(printer)
        .arg(file)
        .output()
        .expect("Failed to print label");
}

pub fn render_small_label_barcode(item: &str, location: &str, team: &str, barcode: &str, image: &str, output: &str){
    let (doc, page, text_layer) = PdfDocument::new("label", Mm(90.0), Mm(31.0), "information");
    // render text layer
    let text_layer = doc.get_page(page).get_layer(text_layer);
    let font = doc.add_external_font(File::open("/usr/share/fonts/TTF/Roboto-Medium.ttf").unwrap()).unwrap();
    text_layer.use_text(String::from("item: ")+item, 10, Mm(2.5),Mm(22.5), &font);
    text_layer.use_text(String::from("location: ")+location, 10, Mm(2.5),Mm(17.5), &font);
    text_layer.use_text(String::from("owner: ")+team, 10, Mm(2.5),Mm(12.5), &font);
    text_layer.use_text(String::from("barcode: ")+barcode, 10, Mm(2.5),Mm(7.5), &font);
    // render barcode image
    let image_layer = doc.get_page(page).add_layer("barcode");
    let image_file = File::open(image).unwrap();
    let mut reader = BufReader::new(image_file);
    let image = printpdf::Image::try_from(printpdf::image::jpeg::JPEGDecoder::new(&mut reader)).expect("failed to read image");
    image.add_to_layer(image_layer.clone(), Some(Mm(45.0)), Some(Mm(5.0)), None, Some(1.3), None, Some(75.0));
    // save the pdf
    doc.save(&mut BufWriter::new(File::create(output).unwrap())).unwrap();
}

pub fn render_small_label(item: &str, location: &str, team: &str, output: &str){
    let (doc, page, text_layer) = PdfDocument::new("label", Mm(90.0), Mm(31.0), "information");
    // render text layer
    let text_layer = doc.get_page(page).get_layer(text_layer);
    let font = doc.add_external_font(File::open("/usr/share/fonts/TTF/Roboto-Medium.ttf").unwrap()).unwrap();
    text_layer.use_text(String::from("item: ")+item, 14, Mm(2.5),Mm(22.0), &font);
    text_layer.use_text(String::from("location: ")+location, 14, Mm(2.5),Mm(14.0), &font);
    text_layer.use_text(String::from("owner: ")+team, 14, Mm(2.5),Mm(6.0), &font);
    // save the pdf
    doc.save(&mut BufWriter::new(File::create(output).unwrap())).unwrap();
}

pub fn render_large_label(item: &str, location: &str, team: &str, barcode: &str, image: &str, output: &str){
    let (doc, page, text_layer) = PdfDocument::new("label", Mm(62.0), Mm(90.0), "information");
    // render text layer
    let text_layer = doc.get_page(page).get_layer(text_layer);
    let font = doc.add_external_font(File::open("/usr/share/fonts/TTF/Roboto-Medium.ttf").unwrap()).unwrap();
    text_layer.use_text(String::from("item: ")+item, 10, Mm(2.5),Mm(22.5), &font);
    text_layer.use_text(String::from("location: ")+location, 10, Mm(2.5),Mm(17.5), &font);
    text_layer.use_text(String::from("owner: ")+team, 10, Mm(2.5),Mm(12.5), &font);
    text_layer.use_text(String::from("barcode: ")+barcode, 10, Mm(2.5),Mm(7.5), &font);
    // render barcode image
    let image_layer = doc.get_page(page).add_layer("barcode");
    let image_file = File::open(image).unwrap();
    let mut reader = BufReader::new(image_file);
    let image = printpdf::Image::try_from(printpdf::image::jpeg::JPEGDecoder::new(&mut reader)).expect("failed to read image");
    image.add_to_layer(image_layer.clone(), Some(Mm(45.0)), Some(Mm(5.0)), None, Some(1.3), None, Some(75.0));

    doc.save(&mut BufWriter::new(File::create(output).unwrap())).unwrap();
}
