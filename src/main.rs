mod html;
mod dom;
mod css;

fn main() {
    let htmlStr = "<html><body>Hello, world!</body></html>".to_string();
    let dom = html::parse(htmlStr);

    println!("{:?}", dom);

    let cssStr = String::from("body { background: red; }");
    let stylesheet = css::parse(cssStr);
    println!("{:?}", stylesheet)
}