
fn main() {
    println!("Hello, world!");

    let mut a=String::from("Aman Souza");
    let mut b=String::from("Amansouza@gmail.com");
    user4(&mut a, &mut b);
    let users=user2(
        String::from("Amah"),
        String::from("emahl")
    );

    let user1 = User {
        name:String::from("Aman ddd"),
        email:String::from("emailtypeshi"),
        number:12345,
    };

    let useru=User{
        number:23456,
        ..user1
    };

    let color1=color(0,0,0);
    println!("{}",color1.0);

    let height=10;
    let width=10;
    let araara=area(height, width);
    println!("The area is {araara}");

    let tup111=(10,10);
    println!("Area of rectangle is {}", area111(tup111));

    let rect1 = Rectangle{
        width:10,
        height:20,
    };

    println!("The struct method gives area {}", area222(&rect1));
    println!("{rect1:#?}");
    dbg!(&rect1);

    let trnle=Triangle{
        height:20,
        breadth:30,
    };

    println!("The area of triangle is {}", trnle.aarea());
}

struct Triangle{
height:u32,
breadth:u32,
}

impl Triangle {
    fn aarea(&self)->u32{
self.height*self.breadth
    }
}

fn area222(rectangle:&Rectangle)->u32{
rectangle.width*rectangle.height
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn area111(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}

fn area(height:u32,width:u32)->u32{
    width*height
}


struct color(u32,u32,u32);

struct User {
    name :String,
    email:String,
    number:u64,
}


fn user4(name1:&mut String, email1:&mut String)->User{
    User{
        name:name1.to_string(),
        email:email1.to_string(),
        number:23456,
    }
}

fn user2(name:String, email:String)->User{
    User{
        name,
        email,
        number:12345
    }
}