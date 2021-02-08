# meta-rust
This is use for store different types of rust program

cargo build -v ==> to see the log

Important Link:
----------------
1> https://doc.rust-lang.org/rustc/codegen-options/index.html  ==> Link for check list of rusc options

2> rustc -C opt-level=z -C lto -C codegen-units=1 -C panic=abort ==> Option need to pass for command line
rustc -Csave-temps hello.rs   ==> This is the command to check intermidate file present in different compilation stage

3> Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. 

4> isize and usize is use for to store cpu order.
5> Signed integers are store in two's complement.
6> RUST does no suppot ++ and -- NNED TO ASK IN COMMUNITY  ==> done
7> In RUST char is 4-byte
8> Functions can use tuples to return multiple values, as tuples can hold any number of values.
9> Array element access same as C.
10> statement means it does not retun anything. And expression means it returns some value.

11> owner ship means we are totaly tranfering the owner which is called move owner. The owner shipexist within the scope and after the scope 
    memory will drop or deallocated. With tranfering owner ship we can use reference & to get the data. This referrence is called borrowing.

12> But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. This code will fail:

13> This restriction allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like.

The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

    Two or more pointers access the same data at the same time.
    At least one of the pointers is being used to write to the data.
    There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem from happening because it won’t even compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

14> Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references occurs before the mutable reference is introduced:

15>
Method Syntax

Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and their first parameter is always self, which represents the instance of the struct the method is being called on.

16> What is the meaning of instance?
ANS: Instance is normaly used in class concept. In calss we have data member and memberfunctions and we use to represent particular object      ----	behaviour and properties. And we are defining one object to mention a particular behaviout in constructure, which is called instance.

17> 
