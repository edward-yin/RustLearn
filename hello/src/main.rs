fn greet() {
    // println!("Hello, world!");
    let soutuh_germ = "Grüß Gott!";
    let chinese = "你好";
    let eng = "hello, world";
    let region = [soutuh_germ, chinese, eng];
    for region in region.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet();
}