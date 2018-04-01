use super::Value;
use std::str::FromStr;
use nom::{crlf, digit, not_line_ending};

/// Core parser implementation.
pub struct Parser;

impl Parser {
    /// Parses RESP from a byte buffer.
    pub fn parse(data: &[u8]) -> Result<(&[u8], Value), ::nom::Err<&[u8]>> {
        named!(
            read_line<&str>,
            do_parse!(line: map_res!(not_line_ending, ::std::str::from_utf8) >> crlf >> (line))
        );

        named!(
            int<i64>,
            do_parse!(
                neg: opt!(tag!("-"))
                    >> value: map_res!(map_res!(digit, ::std::str::from_utf8), i64::from_str)
                    >> (if neg == None { value } else { -value })
            )
        );

        named!(
            parse_int<Value>,
            preceded!(
                tag!(":"),
                do_parse!(datum: int >> crlf >> (Value::Int(datum)))
            )
        );

        named!(
            parse_str<Value>,
            preceded!(
                tag!("+"),
                do_parse!(datum: read_line >> (Value::str(datum)))
            )
        );

        named!(
            parse_err<Value>,
            preceded!(
                tag!("-"),
                do_parse!(datum: read_line >> (Value::err(datum)))
            )
        );

        named!(
            parse_bstr<Value>,
            preceded!(
                tag!("$"),
                do_parse!(
                    len: int >> crlf >> datum: cond_with_error!(len > -1, take_str!(len))
                        >> cond_with_error!(len > -1, crlf)
                        >> (Value::b_str(datum))
                )
            )
        );

        named!(
            parse_array<Value>,
            preceded!(
                tag!("*"),
                do_parse!(
                    len: int >> crlf
                        >> data: cond_with_error!(len > -1, count!(parse_resp, len as usize))
                        >> (Value::Array(data))
                )
            )
        );

        named!(
            parse_resp<Value>,
            alt!(parse_int | parse_str | parse_err | parse_bstr | parse_array)
        );

        parse_resp(data)
    }

    /// Parses a RESP encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lib_resp::Parser;
    /// let age = Parser::parse_str(":-3").unwrap();
    ///
    /// println!("{:?}", age);
    /// ```
    pub fn parse_str(resp: &str) -> Result<Option<Value>, ()> {
        match Parser::parse(resp.as_bytes()) {
            Ok((_i, o)) => Ok(Some(o)),
            Err(e) => {
                if e.is_incomplete() {
                    Ok(None)
                } else {
                    Err(())
                }
            }
        }
    }
}
