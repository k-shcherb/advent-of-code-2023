use std::fs::read_to_string;
use std::path::Path;

fn string_add(input: &str) -> i32 {
    let islice = input.chars();
    let revslice = input.chars().rev();

    let mut sum = Vec::new();


    for i in islice {
        if char::is_numeric(i) {
            sum.push(i);

            break;
        }
    }

    for k in revslice {
        if char::is_numeric(k) {
            sum.push(k);
            break;
        }
    }

    let answer = sum.iter().collect::<String>();

    return str::parse::<i32>(&answer).unwrap();
}


fn part1() -> i32 {
    let input = Path::new("./src/bin/input.txt");
    println!("{}", input.display());
    let mut result = Vec::new();

    for line in read_to_string(input).unwrap().lines() {
        result.push(line.to_string());
    }
    // let mut cnt = 0;
    let mut sum = 0;
    for x in result {
        sum += string_add(&x);
        // println!("{} : {}", x, string_add(&x));

        // if cnt >= 10 {
        //     break;
        // }

    }

    sum
}

fn main() {
    let answer = part1();
    println!("{}", answer);
} 

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn padded() {
        assert_eq!(string_add("1ten1"),11);
    }
     #[test]
    fn midway() {
        assert_eq!(string_add("en2asdf8ffwera"), 28);
    }

    #[test]
    fn onlyone() {
        assert_eq!(string_add("asdflkasdfjlaksdjf7asdfasdf"), 77);
    }

    #[test]
    fn allnums() {
        assert_eq!(string_add("12345"), 15);
    }
}