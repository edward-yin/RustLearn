fn greet() {
    // println!("Hello, world!");
    let soutuh_germ = "Grüß Gott!";
    let chinese = "你好";
    let eng = "hello, world";
    let regions = [soutuh_germ, chinese, eng];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet();
}