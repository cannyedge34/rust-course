fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    let (sum_of_nums, numbers2) = sum(numbers); // we can use numbers.clone() but we do have the performance downside
    let (product_of_nums, numbers3) = product(numbers2); // we can use numbers.clone() but we do have the performance downside
                                                         // we return a tuple from methods which contains
                                                         // the total and the vec of numbers, returning the ownership to this scope
    let average_of_nums = average(numbers3);

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Sum of these numbers: {}", average_of_nums);
}

fn sum(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    (total, numbers)
}

fn product(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    (total, numbers)
}

fn average(numbers: Vec<i64>) -> i64 {
    let length = numbers.len() as i64;
    let (answer, _) = sum(numbers);
    answer / length
}
