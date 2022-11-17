use super::*;

test_json!{
    ConstBool<true> {
        const_bool_true => r"true"
    },
    ConstBool<false> {
        const_bool_false => r"false"
    },
    ConstStr_mutation {
        const_str_mutation => r"mutation"
    }
}
