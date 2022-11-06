// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than 27.
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a hint.


pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {
    name: String,
}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

// Alternative 1:
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

// Alternative 2:
// fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
//     item.some_function() && item.other_function()
// }

// Alternative 3:
// fn some_func<T>(item: T) -> bool where T: SomeTrait + OtherTrait {
//     item.some_function() && item.other_function()
// }

fn main() {}
