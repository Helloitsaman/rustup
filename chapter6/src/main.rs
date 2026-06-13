use std::*;
fn main() {
    println!("Hello, world!");
    let ip4=addrrkind::V4;
    let ip5=addrrkind::V6;

    let home1=ipadd{
        kind:addrrkind::V4,
        address:String::from("127.0.0.1")
    };

    let home2=addrtype::V4(String::from("127.0.0.1"));

    let home3=addy::V4(127,0,0,1);

    let m=Message::Write(String::from("Hello cuh"));
    m.call();

    let state=useState::mumbai;

    let pp=Coin::rupee2(state);
    println!("The value of 50 paise coin is {} paise", value_in_coin(pp));

    let p=String::from("Hello world cusin");
    println!("{p}");

}

#[derive(Debug)]
enum useState{
    mumbai,
    delhi,
}

enum Coin{
    paise50,
    rupee1,
    rupee2(useState),
    rupee5,
}

fn value_in_coin(coin:Coin)->i8{
    match coin{
        Coin::paise50=>50,
        Coin::rupee1=>1,
        Coin::rupee2(state)=>{
            println!("The state it is from is {state:?}");
            2
        },
        Coin::rupee5=>5,
    }
}

enum Option <T> {
    None,
    Some(T),
}

enum Message{
    Quit,
    Move{
        x:u32,y:u32
    },
    Write(String),
    ChangeVolor(i32,i32,i32),
}

impl Message{
    fn call(&self){

    }
}

enum addy {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum addrtype {
    V4(String),
    V6(String),
}

enum addrrkind {
    V4,
    V6,
}

struct ipadd {
    kind:addrrkind,
    address:String,
}