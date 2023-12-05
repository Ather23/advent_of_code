use core::num;
use std::{
    fs::{ self, File, read_to_string },
    io::{ Read, BufReader },
    default,
    collections::{ HashMap, HashSet },
    usize,
};

fn main() {
    let sum: usize = read_to_string("problem2.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let line_text = &String::from(line);
            let (first, last) = get_indices_and_offsets(line_text);
            let first_word = get_word_from_result(line_text, first);
            let last_word = get_word_from_result(line_text, last);
            concact_digits(first_word, last_word)
        })
        .sum(); // gather them

    println!("Answer: {}", sum);
}

fn concact_digits(first: usize, last: usize) -> usize {
    let x = format!("{}{}", first, last);
    return x.parse::<usize>().unwrap();
}

fn get_indices_and_offsets(input: &String) -> ((usize, Option<usize>), (usize, Option<usize>)) {
    let mut cached: HashSet<String> = HashSet::new();
    cached.insert("one".into());
    cached.insert("two".into());
    cached.insert("three".into());
    cached.insert("four".into());
    cached.insert("five".into());
    cached.insert("six".into());
    cached.insert("seven".into());
    cached.insert("eight".into());
    cached.insert("nine".into());

    // idx, offset
    let mut word_result: Vec<(usize, Option<usize>)> = Vec::new();

    let str_leng = input.len();
    for i in 0..str_leng {
        let num_chr = input.chars().nth(i).unwrap();

        if num_chr.is_numeric() {
            word_result.push((i, None));
            continue;
        }

        for j in i + 1..str_leng {
            let chr = input.chars().nth(j);

            if chr.unwrap().is_numeric() {
                // println!("Found a number:{}", &chr.unwrap());
                break;
            }
            let word = input[i..j + 1].to_string();
            // println!("{}-i:{}-j:{}", &word, i, j);

            if cached.contains(&word) {
                word_result.push((i, Some(j)));
            }
        }
    }
    word_result.sort_by(|a, b| a.0.cmp(&b.0));

    // println!("Word result: {:?}", &word_result);
    let first: (usize, Option<usize>) = *word_result.first().unwrap();

    // CONSTRUCT BACK FRMO INDEX

    if word_result.len() > 1 {
        let last: (usize, Option<usize>) = *word_result.last().unwrap();
        return (first, last);
    } else {
        return (first, first);
    }
}

fn get_word_from_result(target: &String, result: (usize, Option<usize>)) -> usize {
    let mut num_word_list: HashMap<String, usize> = HashMap::new();
    num_word_list.insert("one".into(), 1);
    num_word_list.insert("two".into(), 2);
    num_word_list.insert("three".into(), 3);
    num_word_list.insert("four".into(), 4);
    num_word_list.insert("five".into(), 5);
    num_word_list.insert("six".into(), 6);
    num_word_list.insert("seven".into(), 7);
    num_word_list.insert("eight".into(), 8);
    num_word_list.insert("nine".into(), 9);

    let (idx, offset) = result;

    if let Some(offs) = offset {
        let word = target[idx..offs + 1].to_string();
        // println!("Result: {}", word);
        return *num_word_list.get(&word).unwrap();
    } else {
        let num = target.chars().nth(idx).unwrap();
        return num.to_digit(10).unwrap() as usize;
    }
}

fn find_first_and_last_num(x: String) -> usize {
    let mut first_num = false;
    let mut first_num_char: char = char::default();
    let mut sec_num = false;

    let mut result = String::new();
    for i in x.chars() {
        if !i.is_numeric() {
            continue;
        }

        if !first_num {
            first_num_char = i;
            result.push(i);
            first_num = true;
            continue;
        } else if !sec_num {
            result.push(i);
            sec_num = true;
            continue;
        }
        result.pop();
        result.push(i);
    }

    if !sec_num {
        result.push(first_num_char);
    }
    println!("Num: {}", result);

    let num: usize = result.parse::<usize>().unwrap();
    return num;
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;

    #[test]
    fn get_indices_and_offsets_should_return_1() {
        let test_str = "two".into();
        let result = get_indices_and_offsets(&test_str);
        assert!(result.0.0 == 0);
    }

    #[test]
    fn get_indices_and_offsets_should_return_1_and_2() {
        let test_str = "two1".into();
        let result = get_indices_and_offsets(&test_str);
        assert!(result.0.0 == 0);
        assert!(result.1.0 == 3);
    }

    #[test]
    fn get_indices_and_offsets_should_return_two_and_three() {
        let test_str = "threetwo".into();
        let result = get_indices_and_offsets(&test_str);
        assert!(result.0.0 == 0);
        assert!(result.1.0 == 5);
    }

    #[test]
    fn get_indices_and_offsets_should_return_one_two_and_three() {
        let test_str = "three1two".to_string();
        let result = get_indices_and_offsets(&test_str);
        assert!(result.0.0 == 0);
        assert!(result.1.0 == 6);
    }

    #[test]
    fn get_indices_and_offsets_should_return_two_and_nine() {
        let test_str = "two1nine".to_string();
        let result = get_indices_and_offsets(&test_str);
        assert!(result.0.0 == 0);
        assert!(result.1.0 == 4);
    }

    #[test]
    fn get_indices_and_offsets_should_return_7() {
        let test_str = "7pqrstsixteen".to_string();
        let result = get_indices_and_offsets(&test_str);
        assert!(result.0.0 == 0);
        assert!(result.1.0 == 6);
    }

    #[test]
    fn get_word_from_result_should_return_7() {
        let test_str = "7pqrstsixteen".to_string();
        let result = get_word_from_result(&test_str, (0, None));
        println!("##Result: {:?}", result);
        assert!(result == 7);
    }

    #[test]
    fn get_word_from_result_should_return_6() {
        let test_str = "abc6".to_string();
        let result = get_word_from_result(&test_str, (3, None));
        println!("##Result: {:?}", result);
        assert!(result == 6);
    }

    #[test]
    fn get_word_from_result_should_return_two() {
        let test_str = "two".to_string();
        let result = get_word_from_result(&test_str, (0, Some(2)));
        println!("##Result: {:?}", result);
        assert!(result == 2);
    }

    #[test]
    fn get_word_from_result_should_return_three_and_1() {
        let test_str = "threeand1".to_string();
        let result = get_word_from_result(&test_str, (0, Some(4)));
        println!("##Result: {:?}", result);
        assert!(result == 3);
    }

    #[test]
    fn get_indices_and_offsets_test_two1nine() {
        let test_str = "two1nine".to_string();
        let (first, last) = get_indices_and_offsets(&test_str);
        let first_num = get_word_from_result(&test_str, first);
        let last_num = get_word_from_result(&test_str, last);
        assert!(first_num == 2);
        assert!(last_num == 9);
    }

    #[test]
    fn get_indices_and_offsets_test_83() {
        let test_str: String = "eightwothree".to_string();
        let (first, last) = get_indices_and_offsets(&test_str);
        let first_num = get_word_from_result(&test_str, first);
        let last_num = get_word_from_result(&test_str, last);
        assert!(first_num == 8);
        assert!(last_num == 3);
    }

    #[test]
    fn get_indices_and_offsets_test_76() {
        let test_str: String = "7pqrstsixteen".to_string();
        let (first, last) = get_indices_and_offsets(&test_str);
        let first_num = get_word_from_result(&test_str, first);
        let last_num = get_word_from_result(&test_str, last);
        assert!(first_num == 7);
        assert!(last_num == 6);
    }
}
