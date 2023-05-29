/*
    This could be easier with a vector
 */
fn main() {
    let mut times: [usize; 10] = [0, 0, 0, 0, 0, 0 , 0, 0, 0, 0];

    fill_with_random(&mut times);

    println!("{:?}", times);
    println!("amount smaller: {}", amount_smaller(&times, 1500));
    println!("amount equal: {}", amount_equal(&times, 1500));
    println!("amount bigger: {}", amount_bigger(&times, 1500));
    println!("{:?}", deep_copy(&times));
}

fn fill_with_random(array: &mut [usize]) {
    for i in 0..array.len() {
        array[i] = fastrand::usize(1000..2000);
    }
}

fn amount_smaller(array: &[usize], number: usize) -> usize {
    let mut amount = 0;
    /*
        for i in 0..array.len() {
            if array[i] < number {
                amount += 1;
            }
        }
    */
    for value in array {
        if *value < number {
            amount += 1;
        }
    }
    amount
}

fn amount_equal(array: &[usize], number: usize) -> usize {
    let mut amount = 0;

    for value in array {
        if *value == number {
            amount += 1;
        }
    }
    amount
}

fn amount_bigger(array: &[usize], number: usize) -> usize {
    let mut amount = 0;

    for value in array {
        if *value > number {
            amount += 1;
        }
    }
    amount
}

fn deep_copy(array: &[usize]) -> [usize; 10] {
    let mut copy: [usize; 10] = [0, 0, 0, 0, 0, 0 , 0, 0, 0, 0];

    for i in 0..array.len() {
        copy[i] = array[i];
    }
    copy
}
