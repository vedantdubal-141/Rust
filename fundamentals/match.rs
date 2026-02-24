//match vs else..if
//
// match will be checked by the compiler
//
// If a new possibility is added, you will be notified when this occurs
// else..if is not checked by the compiler
//
// If a new possibility is added, your code may contain a bug


fn main() {
    let some_int = 3;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its something else"),
    }

    let some_bool = true;
    match some_bool {
        true => println!("its true"),
        false => println!("its false"),
    }

}


