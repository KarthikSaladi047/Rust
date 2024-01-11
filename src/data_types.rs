// use std::mem;

// pub fn variable_types(){
//     let a: &str = "karthik"; //by default all variables are immutable 
//     println!("Hello, {}", a); // a is immutable

//     // u8,u16,u32,u64,u128,i8,i16,i32,i64,i128
//     let b: u8 = 123; // u=unsigned, 8 bits, 0 to 255
//     println!("b = {}",b);
//     let mut c: i8 = 12; // i=signed, 8 bits, -128 to 127
//     println!("c before = {}",c);
//     c = 20;
//     println!("c after = {}",c);

//     //type-inference = rust will figure out the type of variable
//     let d=13456789;
//     println!("d is {} and takes up {} bytes", d, mem::size_of_val(&d)); // & is pointer
    
//     // usize,isize = size of variables native to OS processer
//     let z: isize = 124;
//     let size_of_z = mem::size_of_val(&z);
//     println!("z is {}, takes {} bytes and my OS is {} bits", z, size_of_z,size_of_z*8);
     
//     //single character (char)
//     let x: char = 'K';
//     println!("{} is a char, size = {} bytes", x, mem::size_of_val(&x));

//     //f32, f64 IEEE754 all Signed! f64 is default float type
//     let f: f64 = -11.0;
//     println!("{}, size = {} bytes", f, mem::size_of_val(&f));

//     //bollean ( true and false)
//     let g: bool = true;
//     println!("{}, size = {} bytes", g, mem::size_of_val(&g));
// }

// /*
// Data Types

// string -> &str = "karthik", "123"
// char -> char = 'S'
// integer -> i8,i16,i32,..,i128  u8,u16,.....,u128    isize,usize (based on OS processor)   unsigned 0 to 255  signed -128 to 127
// float -> f32,f64
// boolean -> bool = true/false
// array -> arr = [1,2,3,4,5]
// tuple -> tuple = ("kar","thik") 
// */