use macro_demo::*;

// The procedural macro will generate a new struct definition with specified fields and methods
#[my_custom_attribute]
struct MyStruct1 {
    name: String,
}

#[delete_attribute]
struct MyStruct2 {
    name: String,
    i1: u64,
}

fn main() {
    let my_struct_1 = MyStruct1::default();
    println!("{:?}", my_struct_1);
    println!("{}", my_struct_1.double_i1());

    let my_struct_2 = MyStruct2{}; // require no fields when initializing
    println!("{:?}", my_struct_2);
}
