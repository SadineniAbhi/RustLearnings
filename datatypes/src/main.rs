fn main() {
    arr_type();
}


fn integers(){
    // types can have different size i means integers and u means unsigned integer
    let x: i32 = 43;
    let y: u8 = 255;
    println!("{} {}",x,y);
}


fn floating(){
    // the type can have different sizes!!
    let x = 8.9;
    let y = 3.4;
}

fn boolean_type(){
    let g = true;
    if g == true{
        println!("g is true");
    }
}


fn char_type() {
    let a: char = 'A';
    let heart_eyed_cat: char = '😻';
    println!("a: {}, heart_eyed_cat: {}", a, heart_eyed_cat);
}


//tuples 

fn tuples_type(){
    let tup = ('a',2,4,"abhijeeth sadineni",9.0);
    println!("{}",tup.0)
}


fn arr_type(){
    let arr = [10,20,30,40,50];
    println!("{}",arr[2]);

    for ele in arr.iter(){
        println!("{}",ele);
    }

}