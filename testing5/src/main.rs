use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    println!("This code consists of guessing game hardcoded as well as a calci");
    println!("This was made without ai");
    let r1=Rectangle{
        length:20,
        width:20,
    };
    println!("The area of rectangle is {}", rect1(r1));

    //guess game from memory but i hardcoded the actual number
loop{
    println!("Enter a random number from 1 to 10");
    let mut guess=String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Some wrong lil bro");

    println!("The guess was {}", guess);

    let guess:u32=guess.trim().parse().expect("Not a number");
    let secret:u32=5;

   match guess.cmp(&secret) {
    Ordering::Less=>println!("Too low of a guess"),
    Ordering::Greater=>println!("Too high of a guess"),
    Ordering::Equal=>{println!("The guess was correct buddy boy");
    break;
}
   };
}

//Calculatoring
loop{
println!("Welcome to the rustsonion calci, enter num 1");
let mut num1=String::new();
io::stdin()
.read_line(&mut num1);

println!("Enter num 2 as well");

let mut num2=String::new();
io::stdin()
.read_line(&mut num2);

let num1:i32=num1.trim().parse().expect("NOT A NUMBER MAN WHTS WRONG WITH U");

println!("What would you like to do with these 2 numbers?????");
let mut operation=String::new();
io::stdin()
.read_line(&mut operation);

let num2:i32=num2.trim().parse().expect("NOT A NUMBER DUDEEE");

match operation.trim() {
    "+"=>{let result=num1+num2;
        println!("Addition?? {} + {} = {}",num1,num2,result);}
    "*"=>{let result=num1*num2;
        println!("multiply?? {} x {} = {}", num1,num2, result);}
    "%"=>{let result=num1/num2;
        println!("divison?? {}/{} = {}", num1,num2,result);}
    "-"=>{let result=num1-num2;
        println!("subtraction?? {} - {} = {}",num1,num2,result);}
    _=>println!("shit ass input"),
};
}

}

struct Rectangle {
    length:u32,
    width:u32
}

fn rect1(rect:Rectangle)->u32{
rect.length*rect.width
}

//guessing game from memory

