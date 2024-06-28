fn main() {
    // owner ship using string data type 
    // let s = String::from("Abhijeeth Sadineni");
    // println!("{}",s);
    // let s2 = s.clone();
    // println!("{}",s2);
    // println!("{}",s);
    //owner ship with functions
    let s = String::from("abhijeeth sadineni");
    let mut z = takes_ownership_and_gives_back(s);
    z.push_str(" is a good programmer");
    println!("{z}");
}


fn takes_ownership(some_string:String){
    println!("{some_string}");
}

fn makes_copy(number: i32){
    println!("{number}");
}

fn gives_ownership(name_str:&str)->String{
    let name = String::from(name_str);
    return name;
}

fn takes_ownership_and_gives_back(some_string:String)->String{
    println!("{some_string}");
    return some_string;
}


// test 

// fn test(){
//     let a = 10;
//     println!("{}",a);
//     // these val are allways deep copy!!!
//     let b = a;
//     println!("{}",a);
//     println!("{}",b);

// }


