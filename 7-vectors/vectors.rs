let mut years: Vec<i32> = vec![1995, 2000, 2005];

years.push(2010);

years.push(2015);

println!("Number of years {}", years.length())

let length: usize = years.length();
// usize can be 32 bit or 64 bit, depending on the system

//Vectors vs arrays
let mut nums: [u8;, 3] = [1, 2, 3];
let mut nums: Vec<u8> = vec![1, 2, 3];

for num in nums {

}