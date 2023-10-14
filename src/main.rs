extern crate getopts;
extern crate image;

use std::default::Default;
use std::fs::File;
use std::io::{BufWriter, Read};

mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod pdf;
mod style;

fn main() {
    // Parse command-line options:
    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png | pdf");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    // println!("{:#?}", matches);
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };
    println!("HTML: {}", &str_arg("f", "png")[..]);

    // Choose a format:
    let png = match &str_arg("f", "png")[..] {
        "png" => true,
        "pdf" => false,
        x => panic!("Unknown output format: {}", x),
    };

    // Read input files:
    let html = read_source(str_arg("h", "examples/test.html"));
    let css = read_source(str_arg("c", "examples/test.css"));

    //　ビューポートは固定。コピートレイと
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    // Parsing and rendering:
    let root_node = html::parse(html); // -> done
    let stylesheet = css::parse(css); // -> done
    let style_root = style::style_tree(&root_node, &stylesheet); // -> done
    // println!("style: {:#?}", style_root);
    let layout_root = layout::layout_tree(&style_root, viewport); // -> no

    // println!("layout: {:#?}", layout_root);

    // Create the output file:
    let filename = str_arg("o", if png { "output.png" } else { "output.pdf" });
    let mut file = BufWriter::new(File::create(&filename).unwrap());

    // Write to the file:
    let ok = if png {
        // println!("viewport: {:#?}", viewport.content); //<- これはコピー前の値
        // ピクセルごとの左上からの色と全体の幅と高さを返す
        let canvas = painting::paint(&layout_root, viewport.content);
        let (w, h) = (canvas.width as u32, canvas.height as u32);
        // ピクセルを元に画像を作成する
        let img = image::ImageBuffer::from_fn(w, h,  |x, y| {
            // println!("x: {}, y: {}", x, y);
            let color = canvas.pixels[(y * w + x) as usize];
            image::Pixel::from_channels(color.r, color.g, color.b, color.a)
        });
        image::ImageRgba8(img).save(&mut file, image::PNG).is_ok()
    } else {
        pdf::render(&layout_root, viewport.content, &mut file).is_ok()
    };
    if ok {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
