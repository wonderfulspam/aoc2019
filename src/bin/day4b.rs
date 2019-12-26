use itertools::Itertools;

const MIN: i32 = 172930;
const MAX: i32 = 683082;

fn main() {
    let valid_passwords = (MIN..MAX)
        .filter(|i| test_password(i))
        .collect::<Vec<i32>>();
    println!("{}", valid_passwords.len());
}

fn test_password(password: &i32) -> bool {
    let pw_vec = number_to_vec(password);
    let mut dd_match = false;
    let mut current_size = 1;
    for (a, b) in pw_vec.into_iter().tuple_windows() {
        if a > b {
            return false;
        }
        if a == b {
            current_size += 1;
        } else {
            if current_size == 2 {
                dd_match = true;
            }
            current_size = 1;
        }
    }
    if current_size == 2 {
        dd_match = true;
    }

    dd_match
}

fn number_to_vec(n: &i32) -> Vec<i32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}
