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

    let kkj=String::from("Whatdup");

    let u=whatup(kkj);
    println!("{u}");

    let st=String::from("Hallo");
    calculatestr(&st);
    println!("The string of {st} is still valid");

    let mut sss=st.clone();
    calculate2(&mut sss);
    println!("{sss} is now valid although we operated on the address and not the string");

    let mut nnn=String::from("Joker");
    {
        let _kop=&mut nnn;
    }
    let _pok=&mut nnn;

    //SLICE AND ITS TYPES
    
}

fn jthu(x:i32){
    let k=String::from("Hello, world!");
    println!("{k} but number is {x}");
}

fn bs(y:String){
    println!("{y} World");
}

fn whatup(jjk:String)->String{
    jjk
}

fn calculatestr(st:&String)->usize{
    st.len()
}

fn calculate2(st:&mut String){
    st.push_str(" WARLD");
}


