fn main() {
    let mut temps: [f64 ; 12] = [-3.4, 0.0, 1.5, 5.2, 10.4, 13.7, 11.3, 17.7, 18.9, 15.1, 0.0, 0.0];

    temps[10] = 17.4;
    temps[11] = 14.2;

    show(&temps);

    println!("avg temp: {}, max temp: {}, min temp: {}", avg_temp(&temps), max_temp(&temps), min_temp(&temps));

    println!("index of max temp: {}", max_temp_index(&temps));

    println!("is 5.2°C warmer than before: {}", warmer(&temps, 3));
}

fn avg_temp(array: &[f64]) -> f64 {
    let mut sum: f64 = 0.0;

    for value in array {
        sum += value;
    }
    sum / array.len() as f64
}

fn max_temp(array: &[f64]) -> f64 {
    let mut max_temp: f64 = -273.15;

    for value in array {
        max_temp = f64::max(max_temp, *value);
    }
    max_temp
}

fn max_temp_index(array: &[f64]) -> usize {
    let mut max_temp: f64 = -f64::MAX;
    let mut i: usize = 0;
    let mut j: usize = 0;

    for value in array {
        if max_temp < *value {
            max_temp = *value;
            i = j;
        }
        j += 1;
    }
    i
}

fn min_temp(array: &[f64]) -> f64 {
    let mut min_temp: f64 = f64::MAX;

    for value in array {
        min_temp = f64::min(min_temp, *value);
    }
    min_temp
}

fn warmer(array: &[f64],index: usize) -> bool {
    let mut warmer: bool = false;

    if array[index] > array[index - 1] {
        warmer = true;
    }
    warmer
}

fn show(array: &[f64]) {
    let mut index: usize = 0;

    for value in array {
        println!("{}: {}", index, value);
        index += 1;
    }
}