#![feature(test)]
extern crate test;
extern crate lib_resp;

use test::Bencher;
use lib_resp::{Parser, Value};

mod bench_parse {
    use super::*;

    #[bench]
    fn int(b: &mut Bencher) {
        let resp = &Value::int(-3).encode_bytes();

        b.iter(|| Parser::parse(resp).unwrap())
    }

    #[bench]
    fn str(b: &mut Bencher) {
        let resp = &Value::str("OK").encode_bytes();

        b.iter(|| Parser::parse(resp).unwrap())
    }

    #[bench]
    fn err(b: &mut Bencher) {
        let resp = &Value::err("ERR").encode_bytes();

        b.iter(|| Parser::parse(resp).unwrap())
    }

    #[bench]
    fn b_str(b: &mut Bencher) {
        let resp = &Value::b_str(Some("foobar")).encode_bytes();

        b.iter(|| Parser::parse(resp).unwrap())
    }

    #[bench]
    fn array(b: &mut Bencher) {
        let resp = &Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar"))
        ])).encode_bytes();

        b.iter(|| Parser::parse(resp).unwrap())
    }
}