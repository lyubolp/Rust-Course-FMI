fn digital_sum(number: u32, base: u32) -> u32{
    let number_of_digits:f64 = (number as f64).log(base as f64) + 1_f64;
    let mut sum: u32 = 0;

    for i in 0..(number_of_digits as u32){
        sum += (number % (base.pow(i+1)) - number % (base.pow(i))) / (base.pow(i));
    }

    sum
}

fn get_digital_root(input: &str, radix: u32) -> Option<u32>{
    let mut current_input_number: u32;

    match u32::from_str_radix(input, radix){
        Ok(value) => current_input_number = value,
        Err(E) => return None,
    };

    current_input_number = digital_sum(current_input_number, radix);
    while current_input_number > radix-1{
        current_input_number = digital_sum(current_input_number, radix);
    }
    Some(digital_sum(current_input_number, radix))
}

/// Десетична бройна система: 0-9
pub fn decimal(input: &str) -> Option<u32> {
    get_digital_root(input, 10)
}

/// Шестнадесетична бройна система: 0-9, последвано от a-f
pub fn hex(input: &str) -> Option<u32> {
    get_digital_root(input, 16)
}

/// Осмична бройна система: 0-7
pub fn octal(input: &str) -> Option<u32> {
    get_digital_root(input, 8)
}

/// Двоична бройна система: 0-1
pub fn binary(input: &str) -> Option<u32> {
    get_digital_root(input, 2)
}

#[cfg(test)]
mod tests {
    use crate::{decimal, hex, octal, binary};

    #[test]
    fn decimal_tests() {
        assert_eq!(decimal("123"), Some(6));
        assert_eq!(decimal("765"), Some(9));
        assert_eq!(decimal("99999999"), Some(9));
        assert_eq!(decimal("5"), Some(5));
        assert_eq!(decimal("asdasdas"), None);
    }

    #[test]
    fn hex_tests(){
        assert_eq!(hex("7b"), Some(3));
        assert_eq!(hex("345"), Some(0xc));
        assert_eq!(hex("a12f"), Some(0xd));
        assert_eq!(hex("5"), Some(5));
        assert_eq!(hex("asdasdas"), None);
    }

    #[test]
    fn octal_tests(){
        assert_eq!(octal("74521"), Some(0o5));
        assert_eq!(octal("345"), Some(0o5));
        assert_eq!(octal("7777777"), Some(0o7));
        assert_eq!(octal("5"), Some(0o5));
        assert_eq!(octal("asdasdas"), None);
    }

    #[test]
    fn binary_tests(){
        assert_eq!(binary("11010010101"), Some(0b1));
        assert_eq!(binary("101010101"), Some(0b1));
        assert_eq!(binary("1"), Some(0b1));
        assert_eq!(binary("0"), Some(0b0));
        assert_eq!(binary("asdasdas"), None);
    }


    #[test]
    fn test_basic() {
        assert_eq!(decimal("345"), Some(3));
        assert_eq!(hex("345"), Some(0xc));

        assert_eq!(octal("1"), Some(1));
        assert_eq!(binary("1"), Some(1));

        let num = String::from("1");
        assert_eq!(binary(&num[..]), Some(1));
    }
}
