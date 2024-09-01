use gpx::read;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use svg::Document;

mod to_svg;

fn read_gpx(file_path: &str) -> gpx::Gpx {
    let file = fs::File::open(file_path).expect("errore durante l'apertura del file");
    let reader = BufReader::new(file);
    let gpx = read(reader).expect("failed to read GPX file");
    gpx
}

fn save_svg(document: Document, output_file: &str) {
    let mut file = File::create(output_file).expect("unable to create file");
    file.write_all(document.to_string().as_bytes())
        .expect("unable to write data");
}

fn main() {
    let input_file = "sample.gpx";
    let output_file = "output.svg";
    let width = 800.0;
    let height = 800.0;
    let gpx = read_gpx(&input_file);
    let document = to_svg::gpx_to_svg(gpx, width, height);
    save_svg(document, &output_file)
}
