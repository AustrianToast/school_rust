pub fn ten_stars_for() {
    for _number in 0..10 {
        println!("*");
    }
}

pub fn squares() {
    let mut result: usize = 1;
    for exponant in 0..20 {
        println!("2 to the power of {exponant} = {result}");
        result *= 2;
    }
}

pub fn squares_to_limit(limit: usize) {
    let mut result: usize = 1;
    for exponant in 0..20 {
        println!("2 to the power of {exponant} = {result}");
        result *= 2;
        if result > limit {
            break;
        }
    }
}

pub fn digit_sum(mut number: usize) {
    let mut digit_sum: usize = 0;
    let mut digit: usize;

    for _i in 0.. {
        if number > 0 {
            digit = number % 10;
            digit_sum += digit;
            number /= 10;
        } else {
            break;
        }
    }
    println!("the sum of all digits is = {}", digit_sum);
}

pub fn roll_dice_till_6() {
    for amount_rolled in 1.. {
        if fastrand::u8(1..7) == 6 {
            println!("rolled {amount_rolled} times");
            break;
        }
    }
}
