fn is_password(p: usize) -> bool
{
    let mut prev_digit: Option<u32> = None;
    let mut has_two_of_the_same = false;
    for digit in p.to_string().chars().map(|d| d.to_digit(10).unwrap()) {
        if let Some(prev) = prev_digit {
            if digit == prev {
                has_two_of_the_same = true;
            }
            if digit < prev {
                return false;
            }    
        }
        prev_digit = Some(digit);
    }
    has_two_of_the_same
}

fn is_password2(p: usize) -> bool
{
    let mut prev_digit: Option<u32> = None;
    let mut group_counter: u32 = 0;
    let mut had_two_group = false;
    for digit in p.to_string().chars().map(|d| d.to_digit(10).unwrap()) {
        if let Some(prev) = prev_digit {
            if digit == prev {
                group_counter += 1
            } else {
                if group_counter == 1 {
                    had_two_group = true;
                }
                group_counter = 0;
            }
            if digit < prev {
                return false;
            }    
        }
        prev_digit = Some(digit);
    }
    had_two_group || group_counter == 1
}

fn main() {
    let num_passwords: usize = (234208..765869).into_iter()
        .filter(|p| is_password(*p))
        .count();

    println!("A: Number of different passwords: {}", num_passwords);

    let num_passwords: usize = (234208..765869).into_iter()
        .filter(|p| is_password2(*p))
        .count();

        println!("B: Number of different passwords: {}", num_passwords);
}
