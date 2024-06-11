fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining =10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count ==2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Edn count = {count}");
    while_loop();
    loop_through_a_collection();
    loop_through_a_collection_for();
    reverse_loop_through_a_collection_for();
}

fn while_loop() {

    println!("
    
This is the start of the While Loop Countdown 
Counting down from 3...");
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_through_a_collection() {
    println!("

This is the start of a loop through an array...
    ");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn loop_through_a_collection_for() {
    println!("
    
This is the start of a loop through an array using a for loop...
    ");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {} but with a 'for'", element);
    }
}

fn reverse_loop_through_a_collection_for() {
    println!("
    
Reverse esreveR...    
    ");

    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!");
}