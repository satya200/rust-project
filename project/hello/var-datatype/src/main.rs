use std::mem;

fn main() {
    let  a = "satya";
    println!("a = {}", a);
    println!("a = {} and size is {}", a, mem::size_of_val(&a));
    //a = 23; // error
    /* But above imutable value a if i use againg let a = 23 it ll work */
    let a = 23;
    //let mut a = 33; // This will give warning becz already "a" become imutable
    println!("2nd time a = {} and size is {}", a, mem::size_of_val(&a));
    let a = 'x';
    println!("3rd time a = {} and size is {}", a, mem::size_of_val(&a));

    let b:i8 = 23;
    println!("1st time b = {} and size is {}", b, mem::size_of_val(&b)); // 1 byte
    let b = 234567789;
    println!("2nd time b = {} and size is {}", b, mem::size_of_val(&b)); // 4 byte

    let c:i8 = 44;
    println!("1st time c = {} and size is {}", c, mem::size_of_val(&c)); // 1 byte
    //c = 4456789; // error 
    //println!("2nd time c = {} and size is {}", c, mem::size_of_val(&c)); //error becz of over flow

    const MAX:u32 = 78; // with const key word we should not use mut and data type should mandatory
    println!("const MAX = {} and size is {}", MAX, mem::size_of_val(&MAX));
    //MAX = 21; // error
    //const MAX:u32 = 90; // reddefine error
    let r#let = 43; //Raw Identifiers  using r# we can use keywords as a varibale
    println!("let = {} and size is {}", r#let, mem::size_of_val(&r#let));
}
