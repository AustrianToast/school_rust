pub mod while_loops;
pub mod for_loops;
pub mod loops;

fn main() {
    all_while_loops();
    all_for_loops();
    all_loops();
}

fn all_while_loops() {
    while_loops::one_star();
    println!();

    while_loops::ten_stars();
    println!();

    while_loops::ten_stars_while();
    println!();

    while_loops::sum_1_to_100();
    println!();

    while_loops::squares();
    println!();

    while_loops::taxes();
    println!();

    let number = 12345;
    while_loops::digit_sum(number);
    println!();

    while_loops::roll_dice_till_6();
}

fn all_for_loops() {
    for_loops::ten_stars_for();
    println!();

    for_loops::squares();
    println!();

    let limit = 127;
    for_loops::squares_to_limit(limit);
    println!();

    let number = 12345; 
    for_loops::digit_sum(number);
    println!();

    for_loops::roll_dice_till_6();
}

fn all_loops() {
    loops::star_line();
    println!();

    loops::star_line_for(10);
    println!();

    loops::star_square_for(5, 5);
    println!();

    loops::star_square_while(5, 5);
    println!();

    loops::star_triangle(5);
    println!();

    loops::star_diagonal(5);
    println!();

    loops::loop1(3, 10);
    println!();

    loops::loop2(3, 10);
    println!();

    loops::loop3(5);
    println!();

    loops::loop4(5);
}
