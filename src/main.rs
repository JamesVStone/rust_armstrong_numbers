use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let num: u32 = args[1].parse().expect("Please enter positive interger!");
    // split number into vector of its composing digits
    let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let length = num.to_string().len();

    //Evaluate number
    let mut sum: u32 = 0;
    let mut working: String = String::from("");
    for digit in &digits {
        sum += digit.pow(length as u32);
        working.push_str(&format!(" {:?}^{:?} ", digit, length));
    }

    // Output result
    if sum == num {
        println!("{0} is an Armstrong number, because: {0} ={1} = {0}", num, working)
    }
    else {
        println!("{0} is not an Armstrong number, because: {0} ={1} = {2}", num, working, sum)
    }
}

/*
9 is an Armstrong number, because 9 = 9^1 = 9
10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
*/
