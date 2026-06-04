fn main() {
    let s=String::from("Hello, world!");
    let mut m=String::from("Hello bird!");
    m.push_str(" No birds");
    println!("{s}");
    println!("{m}");
    let k=7;
    jthu(k);
    println!("{k}");
    let m=String::from("Ahoy");
    println!("{m}");

    let g=s.clone();
    println!("{g}");

    let str=String::from("Hello");
    bs(str);
    println!("{str}");

}

fn jthu(x:i32){
    let k=String::from("Hello, world!");
    println!("{k} but number is {x}");
}

fn bs(y:String){
    println!("{y} World");
}


