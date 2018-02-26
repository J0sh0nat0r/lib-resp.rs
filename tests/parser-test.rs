extern crate nom;
extern crate lib_resp;

use lib_resp::{Value, Parser};

/// Tests the `parse` method
mod test_parse {
    use super::*;

    /// Tests parsing a positive and negative integer
    #[test]
    fn int() {
        assert_eq!(Parser::parse(b":3\r\n").unwrap().1, Value::int(3));
        assert_eq!(Parser::parse(b":-3\r\n").unwrap().1, Value::int(-3));
    }

    /// Tests parsing some simple strings
    #[test]
    fn str() {
        assert_eq!(Parser::parse(b"+OK\r\n").unwrap().1, Value::str("OK"));
        assert_eq!(Parser::parse(b"+ok\r\n").unwrap().1, Value::str("ok"));
    }

    /// Tests parsing some errors
    #[test]
    fn err() {
        assert_eq!(Parser::parse(b"-ERR\r\n").unwrap().1, Value::err("ERR"));
        assert_eq!(Parser::parse(b"-err FOO\r\n").unwrap().1, Value::err("err FOO"));
    }

    /// Tests parsing some bulk strings
    #[test]
    fn b_str() {
        assert_eq!(Parser::parse(b"$-1\r\n").unwrap().1, Value::b_str(None));
        assert_eq!(Parser::parse(b"$0\r\n\r\n").unwrap().1, Value::b_str(Some("")));
        assert_eq!(Parser::parse(b"$3\r\nfoo\r\n").unwrap().1, Value::b_str(Some("foo")));
    }

    /// Tests parsing an array
    #[test]
    fn array() {
        assert_eq!(Parser::parse(b"*-1\r\n").unwrap().1, Value::Array(None));

        assert_eq!(Parser::parse(b"*0\r\n").unwrap().1, Value::Array(Some(Vec::new())));

        assert_eq!(
            Parser::parse(b"*4\r\n:10\r\n+OK\r\n-ERR\r\n$-1\r\n").unwrap().1,

            Value::Array(Some(vec![
                Value::int(10),
                Value::str("OK"),
                Value::err("ERR"),
                Value::b_str(None)
            ]))
        );
    }

    /// These tests ensure erroneous data returns an error
    mod errors {
        use super::*;

        #[test]
        fn int() {
            // Incorrect line endings
            assert!(Parser::parse(b":\rp").is_err());
            assert!(Parser::parse(b":\r\r").is_err());

            // Invalid integers
            assert!(Parser::parse(b":\r\n").is_err());
            assert!(Parser::parse(b":0.3\r\n").is_err());
            assert!(Parser::parse(b":foo\r\n").is_err());
        }

        #[test]
        fn str() {
            // Incorrect line endings
            assert!(Parser::parse(b"+\rp").is_err());
            assert!(Parser::parse(b"+\r\r").is_err());
        }

        #[test]
        fn err() {
            // Incorrect line endings
            assert!(Parser::parse(b"+\rp").is_err());
            assert!(Parser::parse(b"+\r\r").is_err());
        }

        #[test]
        fn b_str() {
            // Incorrect line endings
            assert!(Parser::parse(b"$1\rp").is_err());
            assert!(Parser::parse(b"$1\r\r").is_err());

            // Invalid lengths
            assert!(Parser::parse(b"$\r\n").is_err());
            assert!(Parser::parse(b"$f\r\n").is_err());
            assert!(Parser::parse(b"$f\r\n").is_err());
            assert!(Parser::parse(b"$0.1\r\n").is_err());
        }

        #[test]
        fn array() {
            // Incorrect line endings
            assert!(Parser::parse(b"*1\rp").is_err());
            assert!(Parser::parse(b"*1\r\r").is_err());

            // Invalid lengths
            assert!(Parser::parse(b"*\r\n").is_err());
            assert!(Parser::parse(b"*f\r\n").is_err());
            assert!(Parser::parse(b"*0.1\r\n").is_err());
        }
    }

    /// These tests ensure incomplete data returns IResult::Incomplete
    mod incomplete {
        use super::*;

        #[test]
        fn int() {
            assert!(Parser::parse(b":").unwrap_err().is_incomplete());
            assert!(Parser::parse(b":-").unwrap_err().is_incomplete());
            assert!(Parser::parse(b":-3").unwrap_err().is_incomplete());
            assert!(Parser::parse(b":-3\r").unwrap_err().is_incomplete());
        }

        #[test]
        fn str() {
            assert!(Parser::parse(b"+").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"+O").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"+OK").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"+OK\r").unwrap_err().is_incomplete());
        }

        #[test]
        fn err() {
            assert!(Parser::parse(b"-").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"-E").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"-ER").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"-ERR\r").unwrap_err().is_incomplete());
        }

        #[test]
        fn b_str() {
            assert!(Parser::parse(b"$").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"$2").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"$2\r").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"$2\r\n").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"$2\r\na").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"$2\r\nab").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"$2\r\nab\r").unwrap_err().is_incomplete());
        }

        #[test]
        fn array() {
            assert!(Parser::parse(b"*").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+O").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK\r").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK\r\n").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK\r\n-").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK\r\n-E").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK\r\n-ER").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK\r\n-ERR").unwrap_err().is_incomplete());
            assert!(Parser::parse(b"*2\r\n+OK\r\n-ERR\r").unwrap_err().is_incomplete());
        }
    }
}