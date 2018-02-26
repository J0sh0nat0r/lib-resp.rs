extern crate lib_resp;

use lib_resp::Value;

mod test_encode {
    use super::*;

    #[test]
    fn int() {
        let value = Value::int(-3);

        assert_eq!(value.encode(), ":-3\r\n");
    }

    #[test]
    fn str() {
        let value = Value::str("OK");

        assert_eq!(value.encode(), "+OK\r\n");
    }

    #[test]
    fn err() {
        let value = Value::err("ERR");

        assert_eq!(value.encode(), "-ERR\r\n");
    }

    #[test]
    fn b_str() {
        let value = Value::b_str(None);

        assert_eq!(value.encode(), "$-1\r\n");

        let value = Value::b_str(Some(""));

        assert_eq!(value.encode(), "$0\r\n\r\n");

        let value = Value::b_str(Some("foobar"));

        assert_eq!(value.encode(), "$6\r\nfoobar\r\n");
    }

    #[test]
    fn array() {
        let value = Value::Array(None);

        assert_eq!(value.encode(), "*-1\r\n");

        let value = Value::Array(Some(Vec::new()));

        assert_eq!(value.encode(), "*0\r\n");

        let value = Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar"))
        ]));

        assert_eq!(value.encode(), "*4\r\n:-3\r\n+OK\r\n-ERR\r\n$6\r\nfoobar\r\n")
    }
}

mod test_is_null {
    use super::*;

    #[test]
    fn int() {
        let value = Value::int(-3);

        assert!(!value.is_null());
    }

    #[test]
    fn str() {
        let value = Value::str("OK");

        assert!(!value.is_null());
    }

    #[test]
    fn err() {
        let value = Value::err("ERR");

        assert!(!value.is_null());
    }

    #[test]
    fn b_str() {
        let value = Value::b_str(None);

        assert!(value.is_null());

        let value = Value::b_str(Some(""));

        assert!(!value.is_null());

        let value = Value::b_str(Some("foobar"));

        assert!(!value.is_null());
    }

    #[test]
    fn array() {
        let value = Value::Array(None);

        assert!(value.is_null());

        let value = Value::Array(Some(Vec::new()));

        assert!(!value.is_null());

        let value = Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar"))
        ]));

        assert!(!value.is_null());
    }
}

mod test_is_empty {
    use super::*;

    #[test]
    fn int() {
        let value = Value::int(-3);

        assert!(!value.is_empty());
    }

    #[test]
    fn str() {
        let value = Value::str("");

        assert!(value.is_empty());

        let value = Value::str("OK");

        assert!(!value.is_empty());
    }

    #[test]
    fn err() {
        let value = Value::err("");

        assert!(value.is_empty());

        let value = Value::err("ERR");

        assert!(!value.is_empty());
    }

    #[test]
    fn b_str() {
        let value = Value::b_str(None);

        assert!(value.is_empty());

        let value = Value::b_str(Some(""));

        assert!(value.is_empty());

        let value = Value::b_str(Some("foobar"));

        assert!(!value.is_empty());
    }

    #[test]
    fn array() {
        let value = Value::Array(None);

        assert!(value.is_empty());

        let value = Value::Array(Some(Vec::new()));

        assert!(value.is_empty());

        let value = Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar"))
        ]));

        assert!(!value.is_empty());
    }
}

mod test_clone {
    use super::*;

    #[test]
    fn int() {
        let value = Value::int(-3);

        assert_eq!(value.clone(), value);
    }

    #[test]
    fn str() {
        let value = Value::str("");

        assert_eq!(value.clone(), value);

        let value = Value::str("OK");

        assert_eq!(value.clone(), value);
    }

    #[test]
    fn err() {
        let value = Value::err("");

        assert_eq!(value.clone(), value);

        let value = Value::err("ERR");

        assert_eq!(value.clone(), value);
    }

    #[test]
    fn b_str() {
        let value = Value::b_str(None);

        assert_eq!(value.clone(), value);

        let value = Value::b_str(Some(""));

        assert_eq!(value.clone(), value);

        let value = Value::b_str(Some("foobar"));

        assert_eq!(value.clone(), value);
    }

    #[test]
    fn array() {
        let value = Value::Array(None);

        assert_eq!(value.clone(), value);

        let value = Value::Array(Some(Vec::new()));

        assert_eq!(value.clone(), value);

        let value = Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar"))
        ]));

        assert_eq!(value.clone(), value);
    }
}

mod test_dbg_fmt {
    use super::*;

    #[test]
    fn int() {
        let value = Value::int(-3);

        assert_eq!(format!("{:?}", value), "Int(-3)");
    }

    #[test]
    fn str() {
        let value = Value::str("OK");

        assert_eq!(format!("{:?}", value), "Str(\"OK\")");
    }

    #[test]
    fn err() {
        let value = Value::err("ERR");

        assert_eq!(format!("{:?}", value), "Err(\"ERR\")");
    }

    #[test]
    fn b_str() {
        let value = Value::b_str(None);

        assert_eq!(format!("{:?}", value), "BStr(None)");

        let value = Value::b_str(Some(""));

        assert_eq!(format!("{:?}", value), "BStr(0)");

        let value = Value::b_str(Some("foobar"));

        assert_eq!(format!("{:?}", value), "BStr(6, \"foobar\")");
    }

    #[test]
    fn array() {
        let value = Value::Array(None);

        assert_eq!(format!("{:?}", value), "Array[-1]");

        let value = Value::Array(Some(Vec::new()));

        assert_eq!(format!("{:?}", value), "Array[0]()");

        let value = Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar"))
        ]));

        assert_eq!(
            format!("{:?}", value),
            "Array[4](Int(-3), Str(\"OK\"), Err(\"ERR\"), BStr(6, \"foobar\"))"
        );
    }
}
