fn main() {
    let s=String::from("Hello, world!");
    let mut m=String::from("Hello bird!");
    
    m.push_str(" No birds");
    println!("{s}");
    println!("{m}");
}
