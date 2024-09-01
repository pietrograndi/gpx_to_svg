use core::f64;
use gpx::read;
use gpx::Gpx;
use gpx::TrackSegment;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn read_gpx(file_path: &str) -> gpx::Gpx {
    let file = fs::File::open(file_path).expect("errore durante l'apertura del file");
    let reader = BufReader::new(file);
    let gpx = read(reader).expect("failed to read GPX file");
    gpx
}

fn find_bounds(segment: &TrackSegment) -> (f64, f64, f64, f64) {
    let mut min_lat = f64::INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let mut min_lon = f64::INFINITY;
    let mut max_lon = f64::NEG_INFINITY;

    for point in &segment.points {
        let lon = point.point().x();
        let lat = point.point().y();

        if lat < min_lat {
            min_lat = lat;
        }
        if lat > max_lat {
            max_lat = lat;
        }
        if lon < min_lon {
            min_lon = lon;
        }
        if lon > max_lon {
            max_lon = lon;
        }
    }
    (min_lat, min_lon, max_lat, max_lon)
}

fn add_segment_to_data_quadratic(
    mut data: Data,
    segment: &TrackSegment,
    width: f64,
    height: f64,
) -> Data {
    let (min_lat, min_lon, max_lat, max_lon) = find_bounds(segment);

    let points: Vec<(f64, f64)> = segment
        .points
        .iter()
        .map(|point| {
            let x = (point.point().x() - min_lon) / (max_lon - min_lon) * width;
            let y = (1.0 - (point.point().y() - min_lat) / (max_lat - min_lat)) * height;
            (x, y)
        })
        .collect();

    for i in 0..points.len() {
        if i == 0 {
            data = data.move_to(points[i]);
        } else if i > 1 {
            let mid_x = (points[i - 1].0 + points[i].0) / 2.0;
            let mid_y = (points[i - 1].1 + points[i].1) / 2.0;
            data = data.quadratic_curve_to((points[i - 1].0, points[i - 1].1, mid_x, mid_y));
        }
    }

    data
}

fn add_segment_to_data(mut data: Data, segment: &TrackSegment, width: f64, height: f64) -> Data {
    let (min_lat, min_lon, max_lat, max_lon) = find_bounds(segment);

    // let points: Vec<(f64, f64)> = segment
    //     .points
    //     .iter()
    //     .map(|point| {
    //         let x = (point.point().x() - min_lon) / (max_lon - min_lon) * width;
    //         let y = (1.0 - (point.point().y() - min_lat) / (max_lat - min_lat)) * height;
    //         (x, y)
    //     })
    //     .collect();

    // for i in 0..points.len() {
    //     if i == 0 {
    //         data = data.move_to(points[i]);
    //     } else if i > 1 {
    //         let mid_x = (points[i - 1].0 + points[i].0) / 2.0;
    //         let mid_y = (points[i - 1].1 + points[i].1) / 2.0;
    //         data = data.quadratic_curve_to((points[i - 1].0, points[i - 1].1, mid_x, mid_y));
    //     }
    //
    for (i, point) in segment.points.iter().enumerate() {
        let x = (point.point().x() - min_lon) / (max_lon - min_lon) * width;
        let y = (1.0 - (point.point().y() - min_lat) / (max_lat - min_lat)) * height;

        if i == 0 {
            data = data.move_to((x, y));
        } else {
            data = data.line_to((x, y))
        }
    }

    data
}

fn gpx_to_svg(gpx: Gpx, width: f32, height: f32) -> Document {
    let mut data = Data::new();

    if let Some(track) = gpx.tracks.first() {
        for segment in &track.segments {
            // data = add_segment_to_data_quadratic(data, segment, width as f64, height as f64)
            data = add_segment_to_data_quadratic(data, segment, width as f64, height as f64)
        }
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "#ff0000")
        .set("stroke-width", 10)
        .set("stroke-linecap", "round") // Linee con estremità arrotondate
        .set("stroke-linejoin", "round") // Giunzioni delle linee arrotondate
        .set("d", data);

    Document::new()
        .set("viewBox", (0, 0, width, height))
        .add(path)
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
    let document = gpx_to_svg(gpx, width, height);
    save_svg(document, &output_file)
}
