fn main() {
    let min = 134564;
    let max = 585159;
    let mut current = min;
    let mut count = 0;

    while current <= max {
        if check_password(current) {
            count += 1;
        }
        current += 1;
    }

    println!("Password count:{}", count);
}

fn check_password(password: i32) -> bool {
    let mut valid = false;
    let digits = get_digits(password);
    //println!("digits {:?}", digits);

    let mut duplicate: bool = false;
    let mut increasing: bool = true;
    let mut done = false;
    for i in 0..digits.len()-1 {
        if digits[i] == digits[i+1] {
            if !done {
                duplicate = true;
                if i > 0 {
                    if digits[i-1] == digits[i] {
                        duplicate = false;
                    }
                }
            }
       } else if duplicate {
           done = true;
       }
       if digits[i] > digits[i+1] {
            increasing = false;
            break;
       }
    }

    if duplicate && increasing {
        valid = true;
    }

    return valid;
}

fn get_digits(password:i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();
    let power10 = [100000, 10000, 1000, 100, 10, 1];
    let mut current = password;

    for p10 in power10.iter() {
        let dp10 = *p10;
        //println!("dp10:{}", dp10);
        //println!("current:{}", current);
        if current >= dp10 {
            let digit: i32 = current / dp10;
            //println!("digit:{}", digit);
            digits.push(digit);
            current = current - (dp10 * digit);
        } else {
            digits.push(0);
        }
    }

    return digits;
}
