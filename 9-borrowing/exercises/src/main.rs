//first valid approach

// fn main() {
//     let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

//     let sum_of_nums = sum(&numbers);
//     let product_of_nums = product(&numbers);
//     let average_of_nums = average(&numbers);

//     println!("Sum of these numbers: {}", sum_of_nums);
//     println!("Product of these numbers: {}", product_of_nums);
//     println!("Sum of these numbers: {}", average_of_nums);
// }

// fn sum(numbers: &Vec<i64>) -> i64 {
//     let mut total = 0;

//     for num in numbers.iter() {
//         total += num;
//     }

//     total
// }

// fn product(numbers: &Vec<i64>) -> i64 {
//     let mut total = 1;

//     for num in numbers.iter() {
//         total *= num;
//     }

//     total
// }

// fn average(numbers: &Vec<i64>) -> i64 {
//     let length = numbers.len() as i64;
//     sum(numbers) / length
// }

///////////////////////////////////////////////////////////////////////////////////////////////////
// second valid approach with slices
fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    let sum_of_nums = sum(&numbers);
    let product_of_nums = product(&numbers);
    let average_of_nums = average(&numbers);

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);
}

fn sum(numbers: &[i64]) -> i64 {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    total
}

fn product(numbers: &[i64]) -> i64 {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    total
}

fn average(numbers: &[i64]) -> i64 {
    let length = numbers.len() as i64;
    sum(numbers) / length
}
