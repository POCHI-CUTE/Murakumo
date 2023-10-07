pub mod html;
pub mod dom;

fn main() {
    let htmlStr = "<html><body>Hello, world!</body></html>".to_string();
    let dom = html::parse(htmlStr);

    println!("{:?}", dom);
}