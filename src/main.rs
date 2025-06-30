//use std::io;

// fn main() {
//     println!("Guess the number!");
//     println!("Please input your gues.");
    

//     let mut guess = String::new();

//     io::stdin() 

//         .read_line( &mut guess)
//         .expect("Failed to read line");

//     println!("You Guessed:{}", guess); 


// } 


// fn main(){
//     println!("Guess the number!");
//     println!("Please input your number.");

//     let mut guess = String::new();

//     io::stdin()

//         .read_line(&mut guess)
//         .expect("Failed to read line");
//     println!("You guessed: {}", guess);


// }


// let s: String = String::from("value");


// This is a simple Rust program that demonstrates various data types and their usage.
// It includes examples of boolean, integers, floating point numbers, characters, strings,
// arrays, tuples, and type aliasing.
// The code is structured to show how to declare and use these types in Rust.
// The main function initializes variables of different types and demonstrates accessing elements
// in arrays and tuples, as well as destructuring tuples.

fn main() {
    //bolean
    let b1: bool = true;


    //unsigned integers
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    //signed integers
    let i6: i8 = 1;
    let i7: i16 = 1;
    let i8: i32 = 1;
    let i9: i64 = 1;
    let i10: i128 = 1;

    //floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;


    //platform spesific integers
    let p1: usize = 1;
    let p2: isize =1;



    //characters, &str, and strings
    let c1: char = 'a';
    let s1: &str = "Hello, world!";
    let s2: String = String::from("Hello, World!");


    //arrays 
    let a1: [i32; 5] = [1,2,3,4,5];

    let i1: i32 = a1[1]; // accessing an element in the array


    //tuples
    let t1: (i32, i32, i32) = (1, 2, 3);
    let t1: (i32, f64, &str) = (1, 2.0, "5");

    let s1: &str = t1.2; // accessing an element in the tuple
    let (i1, f1, s1) = t1; // destructuring the tuple

    let unit: () = (); // unit type, similar to void in other languages

    //type aliasing
    type Age = u8;
    let a1: Age = 25; // using the type alias

}

















// mod day2;

// fn main() {
//     //creation 
//     let a: i16 = 5;


//     //mutability
//     let mut b: i32 = 5;
//     b = 10;

//     //shadowing
//     let c: i32 = 10;
//     let c: i32 = 20; // shadowing the previous c

//    // println!("a: {}", a);
//     //println!("b:{}", b);
//     println!("c is: {c}");


//     //scope
//     let d: i32 = 30;
//     {
//         let d: i32 = 40;
//         println!("Inner d is: {d}");
//     }



//     println!("d is: {d}");
//     // {
//     //     let d: i32 = 40; // shadowing the outer d
//     //     println!("Inner d is: {d}");
//     // }




// }

















