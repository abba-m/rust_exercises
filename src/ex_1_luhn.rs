pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut digit_seen = 0;
    let nums: Vec<_> = cc_number.split("").collect();

    
    let mut iter = nums.into_iter().rev().filter(|s| !s.trim().is_empty()).enumerate();
    
    while let Some((idx, val)) = iter.next() {
        if let Ok(parsed_d) = val.parse::<i32>() {
            digit_seen += 1;

            sum += if idx % 2 == 1 {
                let mut dd = parsed_d * 2;
                if dd > 9 {
                    dd -= 9;
                }

                dd
            } else {
                parsed_d
            }
        } else {
            return false;
        }   
    }

    if digit_seen < 2 {
        return false;
    }
    
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}