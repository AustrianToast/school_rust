pub fn one_star() {
    println!("*");
}

pub fn ten_stars() {
    println!("*");
    println!("*");
    println!("*");
    println!("*");
    println!("*");
    println!("*");
    println!("*");
    println!("*");
    println!("*");
    println!("*");
}

pub fn ten_stars_while() {
    let mut i: u8 = 0;
    while i < 10 {
        println!("*");
        i += 1;
    }
}

pub fn sum_1_to_100() {
    let mut i: u8 = 1;
    let mut sum: usize = 0;

    while i <= 100 {
        sum += i as usize;
        i += 1;
    }
    println!("{}", sum);
    println!("{}", (100 * 101) / 2);
}

pub fn squares() {
    let mut exponant: usize = 0;
    let mut result: usize = 1;

    while result <= 1000000000 {
        result *= 2;
        exponant += 1;
        println!("2^{} = {}", exponant, result);
    }
}

pub fn taxes() {
    let mut money: f64 = 100.0;
    let mut years: usize = 0;

    while money < 1000.0 {
        years += 1;
        money *= 1.03;
    }
    println!("after {} years, you have {}€", years, money);
}

pub fn digit_sum(mut number: usize) {
    let mut digit_sum: usize = 0;
    let mut digit: usize;

    while number > 0 {
        digit = number % 10;
        digit_sum += digit;
        number /= 10;
    }
    println!("the sum of all digits is = {}", digit_sum);
}

pub fn roll_dice_till_6() {
    let mut amount_rolled: usize = 1;

    while fastrand::u8(1..7) != 6 {
        amount_rolled += 1;
    }
    println!("rolled {} times", amount_rolled);
}