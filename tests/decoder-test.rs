extern crate lib_resp;

use std::io::BufReader;
use lib_resp::{Decoder, Value};

mod test_decode {
    use super::*;

    #[test]
    fn int() {
        let bytes = Value::int(-3).encode_bytes();

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::int(-3))));
    }

    #[test]
    fn multi_int() {
        let bytes = {
            let mut vec = Vec::new();

            vec.append(&mut Value::int(-3).encode_bytes());
            vec.append(&mut Value::int(-3).encode_bytes());
            vec.append(&mut Value::int(3).encode_bytes());

            vec
        };

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::int(-3))));
        assert_eq!(decoder.decode().ok(), Some(Some(Value::int(-3))));
        assert_eq!(decoder.decode().ok(), Some(Some(Value::int(3))));
    }

    #[test]
    fn str() {
        let bytes = Value::str("OK").encode_bytes();

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::str("OK"))));
    }

    #[test]
    fn multi_str() {
        let bytes = {
            let mut vec = Vec::new();

            vec.append(&mut Value::str("Hello,").encode_bytes());
            vec.append(&mut Value::str(" ").encode_bytes());
            vec.append(&mut Value::str("World!").encode_bytes());

            vec
        };

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::str("Hello,"))));
        assert_eq!(decoder.decode().ok(), Some(Some(Value::str(" "))));
        assert_eq!(decoder.decode().ok(), Some(Some(Value::str("World!"))));
    }

    #[test]
    fn err() {
        let bytes = Value::err("ERR").encode_bytes();

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::err("ERR"))));
    }

    #[test]
    fn multi_error() {
        let bytes = {
            let mut vec = Vec::new();

            vec.append(&mut Value::err("Goodbye,").encode_bytes());
            vec.append(&mut Value::err(" ").encode_bytes());
            vec.append(&mut Value::err("World!").encode_bytes());

            vec
        };

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::err("Goodbye,"))));
        assert_eq!(decoder.decode().ok(), Some(Some(Value::err(" "))));
        assert_eq!(decoder.decode().ok(), Some(Some(Value::err("World!"))));
    }

    #[test]
    fn b_str() {
        // Null BStr
        let bytes = Value::b_str(None::<&str>).encode_bytes();

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::b_str(None::<&str>))));

        // Empty BStr
        let bytes = Value::b_str(Some("")).encode_bytes();

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::b_str(Some("")))));

        // BStr
        let bytes = Value::b_str(Some("foobar")).encode_bytes();

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(
            decoder.decode().ok(),
            Some(Some(Value::b_str(Some("foobar"))))
        );
    }

    #[test]
    fn multi_b_str() {
        let bytes = {
            let mut vec = Vec::new();

            vec.append(&mut Value::b_str(None::<&str>).encode_bytes());
            vec.append(&mut Value::b_str(Some("")).encode_bytes());
            vec.append(&mut Value::b_str(Some("foobar")).encode_bytes());

            vec
        };

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(decoder.decode().ok(), Some(Some(Value::b_str(None::<&str>))));
        assert_eq!(decoder.decode().ok(), Some(Some(Value::b_str(Some("")))));
        assert_eq!(
            decoder.decode().ok(),
            Some(Some(Value::b_str(Some("foobar"))))
        );
    }

    #[test]
    fn array() {
        let bytes = {
            let mut vec = Vec::new();

            vec.append(&mut Value::Array(Some(vec![
                Value::int(-3),
                Value::str("OK"),
                Value::err("ERR"),
                Value::b_str(Some("foobar")),
            ])).encode_bytes());

            vec
        };

        let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

        assert_eq!(
            decoder.decode().ok(),
            Some(Some(Value::Array(Some(vec![
                Value::int(-3),
                Value::str("OK"),
                Value::err("ERR"),
                Value::b_str(Some("foobar")),
            ]))))
        );
    }
}
