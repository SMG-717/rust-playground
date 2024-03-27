fn main() {

    let number = 3;

    // Simple branch
    print!("Number is ");
    if number < 5 {
        println!("small.");
    }
    else {
        println!("BIG!");
    }
    
    // Multiple branches
    print!("Number is also ");
    if number % 2 == 0 { println!("even."); }
    else if number % 3 == 0 { println!("divisible by 3."); }
    else if number % 5 == 0 { println!("divisiblo by cinco."); }
    else if number % 7 == 0 { println!("lucky~"); }
    else { println!("PRIME!"); }
    
    // Branches are assignable!
    let answer = if number > 10 { ">10" } else { "low" };
    println!("The answer is {answer}");
    
    // Loops (also assignable)
    let mut counter = 0;
    let calc = loop {
        counter *= 2;
        counter += 1;
        
        if counter > 24 {
            break counter - 1;
        }
    };
    println!("The calculation resulted in {calc}");

    // Loop Labels and Markers (LLM for short)
    println!("The multiplication table");
    let mut x = 1;
    'mult_table: loop {
        print!("{x:2}: ");
        
        let mut y = 1;

        // Conditional Loops with While
        while y < x {
            print!(" . ");
            y += 1;
        }
        
        loop {
            let prod = x * y;
            print!("{prod:2} ");
            y += 1;

            if y > 10 {
                if x >= 10 {
                    break 'mult_table;
                }
                else {
                    break;
                }
            }
        }

        println!();
        x += 1;
    }
    println!();

    // Iteration Loops with For
    for e in [1, 2, 3, 4, 5] {
        print!("{e}, ");
    }
    println!();
    
    // Reverse Iterating
    for e in (1..4).rev() {
        print!("{e}, ");
    }
    println!("Lift Off!");


}
