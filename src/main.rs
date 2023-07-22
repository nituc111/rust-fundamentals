fn add(num_one: i32, num_two: i32) -> i32 {
    // explicitly type annotate the parameters and value returned
    // we can drop the return keyword when the last line of the function is an expression
    num_one + num_two
}

fn main() {
    let foo = add(10, 5);
    // using the print line macros
    // '!' always appears after macro names, this is how you identify a macro

    // compiler throws errors if you don't pass a string as argument for print line macro, hence we use formatting
    println!("{}", foo);

    // output multiple values
    println!("{} {}", foo, false);

    // supports positional arguments
    // number represents the index of the argument in the argument list
    println!("{0} {0}", foo);

    // complex values
    println!("{:?}", (12, true, "hello"));
}
