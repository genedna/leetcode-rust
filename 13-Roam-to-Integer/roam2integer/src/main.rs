use std::collections::HashMap;

///
/// 
fn main() {
    println!("III - {}", roman_to_int(String::from("III")));
    println!("IV - {}", roman_to_int(String::from("IV")));
    println!("IX - {}", roman_to_int(String::from("IX")));
    println!("LVIII - {}", roman_to_int(String::from("LVIII")));
    println!("MCMXCIV - {}", roman_to_int(String::from("MCMXCIV")));

    println!("III - {}", roman_to_int2(String::from("III")));
    println!("IV - {}", roman_to_int2(String::from("IV")));
    println!("IX - {}", roman_to_int2(String::from("IX")));
    println!("LVIII - {}", roman_to_int2(String::from("LVIII")));
    println!("MCMXCIV - {}", roman_to_int2(String::from("MCMXCIV")));
}

/// LeetCode 平台运行结果
///     1. Runtime: 22 ms, faster than 5.43% of Rust online submissions for Roman to Integer.
///     2. Memory Usage: 2.1 MB, less than 69.42% of Rust online submissions for Roman to Integer.
/// 
/// 代码复杂度 - O(n)
fn roman_to_int(s: String) -> i32 {
    let mut r2i: HashMap<&str, i32> = HashMap::new();
    r2i.insert("I", 1);
    r2i.insert("V", 5);
    r2i.insert("X", 10);
    r2i.insert("L", 50);
    r2i.insert("C", 100);
    r2i.insert("D", 500);
    r2i.insert("M", 1000);
    r2i.insert("IV", 4);
    r2i.insert("IX", 9);
    r2i.insert("XL", 40);
    r2i.insert("XC", 90);
    r2i.insert("CD", 400);
    r2i.insert("CM", 900);

    let mut index: i32 = 0;
    let mut total: i32 = 0;

    while index < s.len() as i32 {
        let mut two_chars: bool = false;
        let mut two_chars_str: String = String::new();

        if index + 1 < s.len() as i32 {
            two_chars_str.push(s.chars().nth(index as usize).unwrap());
            two_chars_str.push(s.chars().nth((index + 1) as usize).unwrap());
            if r2i.contains_key(two_chars_str.as_str()) {
                total += r2i.get(two_chars_str.as_str()).unwrap();
                index += 2;
                two_chars = true;
            }
        }

        if !two_chars {
            let mut one_char_str: String = String::new();
            one_char_str.push(s.chars().nth(index as usize).unwrap());
            total += r2i.get(one_char_str.as_str()).unwrap();
            index += 1;
        }
    }

    total

}

///
/// Runtime: 36 ms, faster than 5.36% of Rust online submissions for Roman to Integer.
/// Memory Usage: 2.1 MB, less than 34.75% of Rust online submissions for Roman to Integer.
/// 
fn roman_to_int2(s: String) -> i32 {
    let mut r2i: HashMap<&str, i32> = HashMap::new();
    r2i.insert("I", 1);
    r2i.insert("V", 5);
    r2i.insert("X", 10);
    r2i.insert("L", 50);
    r2i.insert("C", 100);
    r2i.insert("D", 500);
    r2i.insert("M", 1000);

    let mut index: i32 = s.len() as i32 - 1;
    let mut total: i32 = 0;

    while index >= 0 {
        let mut cur= 0;
        let mut pre: i32 = 0;

        cur = *r2i.get(s.get(index as usize..(index + 1) as usize).unwrap()).unwrap();
        println!("current: {} : {}", s.get(index as usize..(index + 1) as usize).unwrap(), cur);

        if index > 0 {
            pre = *r2i.get(s.get((index - 1) as usize..index as usize).unwrap()).unwrap();
            println!("previous: {} : {}", s.get((index - 1) as usize..index as usize).unwrap(), pre);

            if pre < cur {
                total += cur - pre;
                index -= 2;
            } else {
                total += cur;
                index -= 1;
            }
        } else {
            total += cur;
            index -= 1;
        }
    }

    total
}