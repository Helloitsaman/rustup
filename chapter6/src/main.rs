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