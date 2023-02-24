fn main() {
    let mut city_names = vec!["Rubynia", "Javasburg", "Badajoz"];

    // rust does not have a concept of undefined or nil, that's why
    // we use this option enum. Represents the concept of might or might not
    // be there, if the vec! is empty for example.
    // rust forces you check edgecases scenarios
    let last_city = match city_names.pop() {
        Some(inner_value) => inner_value,
        None => "",
    };

    if last_city.starts_with("B") {
        println!("{} starts with B", last_city);
    } else {
        println!("{} does not start with B", last_city);
    }

    city_names.push(last_city);

    println!("Here is the full list of cities");

    for city_name in city_names.iter() {
        println!("* {}", city_name);
    }
}
