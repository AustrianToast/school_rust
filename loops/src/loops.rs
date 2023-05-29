pub fn star_line() {
    println!("**********");
}

pub fn star_line_for(star_amount: usize) {
    for _i in 0..star_amount {
        print!("*");
    }
    println!();
}

pub fn star_square_for(line_amount: usize, column_amount: usize) {
    for _i in 0..line_amount {
        for _j in 0..column_amount {
            print!("*");
        }
        println!();
    }
}

pub fn star_square_while(line_amount: usize, column_amount: usize) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    
    while i < line_amount {
        while j < column_amount {
            print!("*");
            j += 1;
        }
        j = 0;
        println!();
        i += 1;
    }
}

pub fn star_triangle(line_amount: usize) {
    for i in 1..(line_amount + 1) {
        for _j in 0..i {
            print!("*");
        }
        println!();
    }
}

pub fn star_diagonal(length: usize) {
    for i in 1..(length + 1) {
        for _j in 0..(i - 1) {
            print!(" ");
        }
        println!("*");
    }
}

pub fn loop1(mut limit1: usize, mut limit2: usize) {
    let temp_value: usize;

    if limit1 > limit2 {
        temp_value = limit1;
        limit1 = limit2;
        limit2 = temp_value;
    }

    for i in limit1..limit2 {
        println!("{i}");
    }
}

pub fn loop2(limit1: usize, limit2: usize) {
    let upper_limit: usize;
    let lower_limit: usize;

    if limit1 < limit2 {
        lower_limit = limit1;
        upper_limit = limit2;
    } else {
        lower_limit = limit2;
        upper_limit = limit1;
    }

    for i in lower_limit..upper_limit {
        println!("{i}");
    }
}

pub fn loop3(limit: isize) {
    let variable: isize;
    let mut output: isize = 0;

    if limit > 0 {
        variable = 1;
    } else {
        variable = -1;
    }

    for _i in 0.. {
        println!("{output}");
        output += variable;
        if output == (limit + variable) {
            break;
        }
    }
}

pub fn loop4(amount: usize) {
    let mut zaehler: isize = -2;
    let mut nenner: isize = 10;
    let mut nenner_old: isize = 5;
    let mut nenner_new: isize;

    for _i in 0..amount {
        zaehler *= -2;
        nenner_new = nenner + nenner_old;
        nenner_old = nenner;
        nenner = nenner_new;
        println!("{zaehler}/{nenner}");
    }
}