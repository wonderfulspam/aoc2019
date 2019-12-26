const MIN: i32 = 172930;
const MAX: i32 = 683082;

fn main() {
    let valid_passwords = (MIN..MAX)
        .filter(|i| test_password(i))
        .collect::<Vec<i32>>();
    println!("{}", valid_passwords.len());
}

fn test_password(password: &i32) -> bool {
    let mut pw_string = number_to_vec(password);
    let original_len = pw_string.len();
    pw_string.dedup();
    if original_len == pw_string.len() {
        return false;
    }
    for i in 1..pw_string.len() {
        if pw_string[i] < pw_string[i - 1] {
            return false;
        }
    }
    true
}

fn number_to_vec(n: &i32) -> Vec<i32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}
