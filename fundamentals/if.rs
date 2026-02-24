fn main(){
    // correct
    let a = 99;
    if (a > 200) {
        println!("Huge number")
    } else if (a > 99) {
        println!("Big number")  // This will execute for `a=99`
    } else {
        println!("Small number")
    }

    //incorrect
    let a = 99;
    if (a > 99) {  // First condition fails, so it skips the next check
        println!("Big number")  // This will NOT execute for `a=99`
    }
    else if (a > 200) {
        println!("Huge number")  // This will also NOT execute for `a=99`
    }
    else {
        println!("Small number")  // This executes, but the logic is flawed
    }

}