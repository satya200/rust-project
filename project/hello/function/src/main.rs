fn foo(x: i32) -> i32 {
    println!("In side foo() and val = {}", x);
    5 // In rust this is new rust type
}
fn main() {
    println!("Hello, world!");
    //let res = foo(5);
    foo(5);
    //println!("foo ret val = {}", res);
    let y = {
	let x = 3;
	x + 1   //Here if we put semi colon then ot became statement and in RUST statement never return only expression return
    };
    println!("y val = {}", y);
}
