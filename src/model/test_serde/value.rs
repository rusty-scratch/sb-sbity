use super::*;

test_json!{
    Name {
       name => r#""johndoe""#
    }
    
    Number {
        number1 => "25",
        number2 => "-25",
        number3 => "234.23498327",
        number4 => "-234.23498327"
    }

    Value {
        value_int_string => r#""230""#,
        value_float_string => r#""69.8""#,
        value_string => r#""double door""#,
        value_number1 => "0",
        value_number2 => "25",
        value_number3 => "-40",
        value_number4 => "89.56984",
        value_number5 => "-23948.2391"
    }
}