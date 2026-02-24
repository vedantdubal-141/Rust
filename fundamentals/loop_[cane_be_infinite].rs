fn main() {
    let mut a =0; // must be mutable unless it will be infinite loop because a won't increase

    loop {
        if a==5{
            break;
        }

        println!("{:?}",0);
        a = a+1;
        println!("{:?}",a)
    }

}