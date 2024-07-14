#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod data_types;
mod operators;
mod scope_and_shadowing;
mod stack_heap;
mod if_statements;
mod while_loop;
mod for_loop;
mod match_statement;
mod combination_lock;

fn main(){
    // data_types::variable_types();
    // operators::operators();
    // scope_and_shadowing::scope_and_shadowing();
    // stack_heap::stack_heap();
    // if_statements::if_statement();
    // while_loop::while_loop();
    // for_loop::for_loop();
    // match_statement::match_statement();
    combination_lock::combination_lock();
}


// const AGE:u8 = 23; //no fixed address, globally available, must specify type and variable has to be in upeercase snake case notation
// static mut LAST_YEAR:u32 = 2023;
// static NEW_YEAR:u32 = 2024;
// fn main() {

//     println!("age is {}", AGE);
//     unsafe  {
//         println!("good bye {}", LAST_YEAR);
//     }
//     println!("welcome {}", NEW_YEAR);
// }
