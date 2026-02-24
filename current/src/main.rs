fn sum(a:i32,b:i32 ) -> i32 {
    return a+b;
}

fn d_result(result: i32){
    println!("{}", result);
}
fn main() {
    let result = sum(2,3);
    d_result(result);
}
