
/*Structs: A type that contains multiple pieces of data

All or nothing – cannot have some pieces of data and not others
-Each piece of data is called a “field”
-Makes working with data easier
-Similar data can be grouped together
*/

struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

let my_box = ShippingBox {
depth: 3,
width: 2,
height: 5,
};

let tall = my_box.height;
println!("the box is {:?} units tall", tall);

/*
-Structs deal with multiple pieces of data
-All fields must be present to create a struct
-Fields can be accessed using a dot (.)
*/