# Rust: Armstrong Numbers

My GCI challenge submission for a rust program which validates armstrong numbers.

# Usage

Compile and run:
`cargo run` + `<number which you want to validate>`

The program will output in this form:
```
9 is an Armstrong number, because 9 = 9^1 = 9
10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 1
```