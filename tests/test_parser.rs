use oneline_template::template::Template;
use serde::Serialize;

fn assert_ok<T>(format: &str, value: &T, expected: &str)
    where T: Serialize
{
    let template = Template::parse(format).expect("Unable to create template");
    let value = template.serialize(value).expect("Unable to get value formatted by template");
    assert_eq!(value, expected);
}

fn assert_template_ok(format: &str) {
    let template = Template::parse(format);
    assert!(template.is_ok());    
}

fn assert_template_err(format: &str, expected: &str) {
    let template = Template::parse(format);
    let error = template.unwrap_err();
    let error = format!("{}", error);
    assert_eq!(error, expected);
}

#[derive(serde_derive::Serialize)]
struct First {
    second: Second,
}

#[derive(serde_derive::Serialize)]
struct Second {
    third: Third,
}

#[derive(serde_derive::Serialize)]
struct Third {
    str_value: &'static str,
    string_value: String,
    bool_value: bool,
    bytes_value: Vec<u8>,
    char_value: char,
    uint_value: u64,
    int_value: i64,
    float_value: f64,
}

fn create_object() -> First {
    let third = Third {
        str_value: "str_value",
        string_value: "string_value".into(),
        bool_value: false,
        bytes_value: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21],
        char_value: 'c',
        uint_value: 99,
        int_value: -55,
        float_value: 42.5f64,
    };
    let second = Second {
        third,
    };
    return First {
        second,
    }
}

#[test]
fn test_parser_ok() {
    let object = create_object();
    assert_ok("{self.second.third.str_value}", &object, "str_value");
    assert_ok("{self.second.third.string_value}", &object, "string_value");
    assert_ok("{self.second.third.bool_value|bool:to_string()}", &object, "false");
    assert_ok("{self.second.third.bytes_value}", &object, "Hello, world!");
    assert_ok("{self.second.third.char_value}", &object, "c");
    assert_ok("{self.second.third.uint_value}", &object, "99");
    assert_ok("{self.second.third.int_value}", &object, "-55");
    assert_ok("{self.second.third.float_value|float:to_string()}", &object, "42.5");
}

#[test]
fn test_parser_ok_with_spaces() {
    let object = create_object();
    assert_ok("{ self.second.third.str_value }", &object, "str_value");
    assert_ok("{ self.second.third.string_value }", &object, "string_value");
    assert_ok("{ self.second.third.bool_value|bool:to_string() }", &object, "false");
    assert_ok("{ self.second.third.bytes_value }", &object, "Hello, world!");
    assert_ok("{ self.second.third.char_value }", &object, "c");
    assert_ok("{ self.second.third.uint_value }", &object, "99");
    assert_ok("{ self.second.third.int_value }", &object, "-55");
    assert_ok("{ self.second.third.float_value|float:to_string() }", &object, "42.5");
}

#[test]
fn test_parser_err_field_1() {
    assert_template_err("{}", "Unexpected input `}`");
    assert_template_err("{ }", "Unexpected input `}`");
}

#[test]
fn test_parser_err_field_2() {
    assert_template_err("{ self .second.third.str_value }", "Expected token `}` in `.second.third.str_value }`");
    assert_template_err("{ self. second.third.str_value }", "Wrong field name");
    assert_template_err("{ self.second .third.str_value }", "Expected token `}` in `.third.str_value }`");
    assert_template_err("{ self.second. third.str_value }", "Wrong field name");
    assert_template_err("{ self.second.third .str_value }", "Expected token `}` in `.str_value }`");
    assert_template_err("{ self.second.third. str_value }", "Wrong field name");
}

#[test]
fn test_parser_err_field_3() {
    assert_template_err("{self#second.third.str_value}", "Expected token `}` in `#second.third.str_value}`");
    assert_template_err("{self.second#third.str_value}", "Expected token `}` in `#third.str_value}`");
    assert_template_err("{self.second.third#str_value}", "Expected token `}` in `#str_value}`");
    assert_template_err("{self.second.third.str_value#}", "Expected token `}` in `#}`");
}

#[test]
fn test_parser_err_process_1() {
    assert_template_err("{self.second.third.str_value |}", "Wrong function name");    
    assert_template_err("{self.second.third.str_value | }", "Wrong function name");    
    assert_template_err("{self.second.third.str_value | @}", "Wrong function name");    
    assert_template_err("{self.second.third.str_value | (}", "Wrong function name");    
    assert_template_err("{self.second.third.str_value | ()}", "Wrong function name");    
}

#[test]
fn test_parser_err_process_2() {
    assert_template_err("{self.second.third.str_value | uint:to_string}", "Expected token `(` in `}`");
    assert_template_err("{self.second.third.str_value | uint:to_string }", "Expected token `(` in `}`");
    assert_template_err("{self.second.third.str_value | uint:to_string( }", "Invalid argument `` in input `}`");
    assert_template_err("{self.second.third.str_value | uint:to_string ( }", "Invalid argument `` in input `}`");
    assert_template_ok("{self.second.third.str_value | uint:to_string()}");
    assert_template_ok("{self.second.third.str_value | uint:to_string () }");
    assert_template_ok("{self.second.third.str_value | uint:to_string ( ) }");
}

#[test]
fn test_parser_err_process_3() {
    assert_template_err("{self.second.third.str_value | uint:to_string() | uint:to_string}", "Expected token `(` in `}`");
    assert_template_err("{self.second.third.str_value | uint:to_string() | uint:to_string }", "Expected token `(` in `}`");
    assert_template_err("{self.second.third.str_value | uint:to_string() | uint:to_string( }", "Invalid argument `` in input `}`");
    assert_template_err("{self.second.third.str_value | uint:to_string() | uint:to_string ( }", "Invalid argument `` in input `}`");
    assert_template_ok("{self.second.third.str_value | uint:to_string() | uint:to_string()}");
    assert_template_ok("{self.second.third.str_value | uint:to_string() | uint:to_string () }");
    assert_template_ok("{self.second.third.str_value | uint:to_string() | uint:to_string ( ) }");
}

#[test]
fn test_parser_argument() {
    assert_template_ok("{self.second.third.str_value | uint:hex_fmt(50uint) }");
    assert_template_err("{self.second.third.str_value | uint:hex_fmt(50int) }", "Argument with index `0` at function `uint:hex_fmt` has wrong type");
    assert_template_ok("{self.second.third.str_value | string:unwrap_or('default_value') }");
    assert_template_ok("{self.second.third.str_value | string:unwrap_or('default_\\'value') }");
    assert_template_ok("{self.second.third.str_value | bool:unwrap_or(true) }");
    assert_template_ok("{self.second.third.str_value | bool:unwrap_or(false) }");
}