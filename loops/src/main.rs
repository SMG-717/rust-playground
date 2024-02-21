fn main() {
    _example4();
}

fn _example1() {
    let mut v = 0; 
    let v = loop {
        v += 1;
     
     
     fn _example4() {
        for e in [1, 2, 3, 4, 5] {
            println!("{e}");
        }
     }   if v >= 10 {
            break v * 2;
        }
    };
    println!("Value: {v}");
}

fn _example2() {
    let mut count = 0;
    let count = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count * 3;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count}");
}

fn _example3() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
}

fn _example4() {
    for e in 1..=5 {
        println!("{e}");
    }
}