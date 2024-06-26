fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{r1}");
    println!("the values is {r1}");
    

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}