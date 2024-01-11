// pub fn scope_and_shadowing(){
//     let a = 111;

//     //scope a variable lies within flower brackets
//     {
//         let b = 222;
//         let a= 333;
//         println!("inside b = {}", b);
//         println!("inside a = {}", a); // we can access the variable a from outer scope if that variable is not defined within the scope - this is called shadowing
//     }
//      println!("outside a = {}", a);
//      //println!("outside b = {}", b); //generate error

//      let a = 444;
//      println!("redeclared a = {}", a); // if we redeclare same variable, the variable will take newly declared value, from that point of code.
// }