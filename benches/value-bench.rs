#![feature(test)]
extern crate lib_resp;
extern crate test;

use test::Bencher;
use lib_resp::Value;

mod bench_encode {
    use super::*;

    /// Benchmarks encoding an integer
    #[bench]
    fn int(b: &mut Bencher) {
        let value = Value::int(-3);

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding a simple string
    #[bench]
    fn str(b: &mut Bencher) {
        let value = Value::str("OK");

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding an error
    #[bench]
    fn err(b: &mut Bencher) {
        let value = Value::err("ERR");

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding a bulk string
    #[bench]
    fn b_str(b: &mut Bencher) {
        let value = Value::b_str(Some("foobar"));

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding an empty bulk string
    #[bench]
    fn empty_b_str(b: &mut Bencher) {
        let value = Value::b_str(Some(""));

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding a null bulk string
    #[bench]
    fn null_b_str(b: &mut Bencher) {
        let value = Value::b_str(None);

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding an array
    #[bench]
    fn array(b: &mut Bencher) {
        let value = Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar")),
            Value::Array(Some(Vec::new())),
        ]));

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding an empty array
    #[bench]
    fn empty_array(b: &mut Bencher) {
        let value = Value::Array(Some(Vec::new()));

        b.iter(|| value.encode())
    }

    /// Benchmarks encoding a null array
    #[bench]
    fn null_array(b: &mut Bencher) {
        let value = Value::Array(None);

        b.iter(|| value.encode())
    }
}
