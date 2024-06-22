fn main(){
   using_while();
   using_loop();
   using_for();
   nested_loops();
}


fn using_loop(){
    let mut count = 0;
    loop{
        count+=1;
        println!("{}",count);
        if count == 5{
            break;
        }
    }
}


fn using_while(){
    let mut count = 10;
    while count != 0{
        println!("{}",count);
        count -= 1;
    }

}

fn using_for(){
    let a = [10,20,30,40];
    for ele in a.iter(){
        println!("{}",ele);
    }

    for number in 100..111{
        println!("{}",number);
    }
}


fn nested_loops(){
    let mut counter = 0;
    loop {
        for number in 1..6{
            println!("{}",number);
        }
        if counter == 5{
            break;
        }
        counter+=1;
    }
}