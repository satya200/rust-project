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

11> 
