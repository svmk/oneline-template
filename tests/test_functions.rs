use oneline_template::template::Template;
use serde::Serialize;

fn assert_ok<T>(format: &str, value: &T, expected: &str)
    where T: Serialize
{
    let template = Template::parse(format).expect("Unable to create template");
    let value = template.serialize(value).expect("Unable to get value formatted by template");
    assert_eq!(value, expected);
}

fn assert_err<T>(format: &str, value: &T, expected: &str)
    where T: Serialize
{
    let template = Template::parse(format).expect("Unable to create template");
    let error = template.serialize(value).expect_err("Unable to get value formatted by template");
    let error = format!("{}", error);
    assert_eq!(error, expected);
}

#[test]
fn test_bool() {
    assert_err("{self}", &true, "Convertation bool to string is not supported");
    assert_err("{self}", &false, "Convertation bool to string is not supported");
}

#[test]
fn test_func_bool_to_string() {
    assert_ok("{self|bool:to_string()}", &true, "true");
    assert_ok("{self|bool:to_string()}", &false, "false");
}

#[test]
fn test_func_bool_unwrap_or() {
    let value: Option<bool> = None;
    assert_ok("{self|bool:unwrap_or(true)|bool:to_string()}", &value, "true");
}

#[test]
fn test_float() {
    assert_err("{self}", &0.5f32, "Convertation float to string is not supported");
    assert_err("{self}", &0.5f64, "Convertation float to string is not supported");
}

#[test]
fn test_func_float_to_string() {
    assert_ok("{self|float:to_string()}", &0.5f32, "0.5");
    assert_ok("{self|float:to_string()}", &0.5f64, "0.5");
}

#[test]
fn test_int() {
    assert_ok("{self}", &-5, "-5");
    assert_ok("{self}", &5i32, "5");
}

#[test]
fn test_func_int_abs() {
    assert_ok("{self|int:abs()}", &-5i32, "5");
    assert_ok("{self|int:abs()}", &5i32, "5");
}

#[test]
fn test_func_int_hex() {
    assert_ok("{self|int:hex()}", &-5i32, "fffffffffffffffffffffffffffffffb");
    assert_ok("{self|int:hex()}", &5i32, "5");
}

#[test]
fn test_func_int_hex_fmt() {
    assert_ok("{self|int:hex_fmt(100uint)}", &-5i32, "00000000000000000000000000000000000000000000000000000000000000000000fffffffffffffffffffffffffffffffb");
    assert_ok("{self|int:hex_fmt(100uint)}", &5i32, "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005");
}

#[test]
fn test_func_int_to_string() {
    assert_ok("{self|int:to_string()}", &5i32, "5");
    assert_ok("{self|int:to_string()}", &-5i32, "-5");
}

#[test]
fn test_string() {
    let value = "abc";
    assert_ok("{self}", &value, "abc");
}

#[test]
fn test_func_string_substr() {
    let value = "2020-12-01";
    assert_ok("{self|string:substr(0uint, 4uint)|string:unwrap_or('----')}", &value, "2020");
    assert_ok("{self|string:substr(5uint, 2uint)|string:unwrap_or('--')}", &value, "12");
    assert_ok("{self|string:substr(100uint, 0uint)|string:unwrap_or('--')}", &value, "--");
    assert_ok("{self|string:substr(100uint, 0uint)|string:unwrap_or('-\\'-')}", &value, "-'-");
}

#[test]
fn test_func_string_trim() {
    let value = "\t 2020-12-01\t ";
    assert_ok("{self|string:trim()}", &value, "2020-12-01");
}

#[test]
fn test_func_string_trim_end() {
    let value = "\t 2020-12-01\t ";
    assert_ok("{self|string:trim_end()}", &value, "\t 2020-12-01");
}

#[test]
fn test_func_string_trim_start() {
    let value = "\t 2020-12-01\t ";
    assert_ok("{self|string:trim_start()}", &value, "2020-12-01\t ");
}

#[test]
fn test_uint() {
    assert_ok("{self}", &5u32, "5");
}

#[test]
fn test_func_uint_hex() {
    assert_err("{self|uint:hex()}", &-5i32, "Trying to cast Int(-5) as uint");
    assert_err("{self|uint:hex()}", &5i32, "Trying to cast Int(5) as uint");
    assert_ok("{self|uint:hex()}", &5u32, "5");
}

#[test]
fn test_func_uint_hex_fmt() {
    assert_err("{self|uint:hex_fmt(100uint)}", &-5i32, "Trying to cast Int(-5) as uint");
    assert_err("{self|uint:hex_fmt(100uint)}", &5i32, "Trying to cast Int(5) as uint");
    assert_ok("{self|uint:hex_fmt(100uint)}", &5u32, "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005");
}

#[test]
fn test_func_uint_to_string() {
    assert_err("{self|uint:to_string()}", &5i32, "Trying to cast Int(5) as uint");
    assert_err("{self|uint:to_string()}", &-5i32, "Trying to cast Int(-5) as uint");
    assert_ok("{self|uint:to_string()}", &5u32, "5");
}

#[test]
fn test_func_debug_type() {
    assert_ok("{self|debug:type()}", &"abc", "String");
    assert_ok("{self|debug:type()}", &true, "Bool");
    let value = vec![1u8];
    assert_ok("{self|debug:type()}", &value, "Bytes");
    assert_ok("{self|debug:type()}", &'a', "Char");
    let value: Option<String> = None;
    assert_ok("{self|debug:type()}", &value, "Option<..>");
    let value: Option<String> = Some("abc".into());
    assert_ok("{self|debug:type()}", &value, "Option<String>");
    assert_ok("{self|debug:type()}", &5u32, "UInt");
    assert_ok("{self|debug:type()}", &5i32, "Int");
    assert_ok("{self|debug:type()}", &35.0f64, "Float");
}