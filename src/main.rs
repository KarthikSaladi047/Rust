#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod data_types;
mod operators;
mod scope_and_shadowing;


const AGE:u8 = 23; //no fixed address, globally available, must specify type and variable has tio be in upeercase snake case notation
static mut LAST_YEAR:u32 = 2023;
static NEW_YEAR:u32 = 2024;
fn main() {
    // data_types::variable_types();
    // operators::operators();
    // scope_and_shadowing::scope_and_shadowing();
    println!("age is {}", AGE);
    unsafe  {
        println!("good bye {}", LAST_YEAR);
    }
    println!("welcome {}", NEW_YEAR);
}
