use combine::parser::byte::byte;
use combine::parser::range::{recognize_with_value, take_while1};
use combine::stream::RangeStream;
use combine::*;

use crate::key::Key;
use crate::parser::strings::{basic_string, literal_string};
use crate::parser::trivia::{from_utf8_unchecked, ws};
use crate::repr::{Decor, Repr};
use crate::InternalString;

// key = simple-key / dotted-key
// dotted-key = simple-key 1*( dot-sep simple-key )
parse!(key() -> Vec<Key>, {
    sep_by1(
        attempt((
            ws(),
            simple_key(),
            ws(),
        )).map(|(pre, (raw, key), suffix)| {
            Key::new(key).with_repr_unchecked(Repr::new_unchecked(raw)).with_decor(Decor::new(pre, suffix))
        }),
        byte(DOT_SEP)
    ).expected("key")
});

// simple-key = quoted-key / unquoted-key
// quoted-key = basic-string / literal-string
parse!(simple_key() -> (&'a str, InternalString), {
    recognize_with_value(
        look_ahead(any()).then(|e| {
            dispatch!(e;
                crate::parser::strings::QUOTATION_MARK => basic_string().map(|s: String| s.into()),
                crate::parser::strings::APOSTROPHE => literal_string().map(|s: &'a str| s.into()),
                _ => unquoted_key().map(|s: &'a str| s.into()),
            )
        })
    ).map(|(b, k)| {
        let s = unsafe { from_utf8_unchecked(b, "If `quoted_key` or `unquoted_key` are valid, then their `recognize`d value is valid") };
        (s, k)
    })
});

// unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
parse!(unquoted_key() -> &'a str, {
    take_while1(is_unquoted_char).map(|b| {
        unsafe { from_utf8_unchecked(b, "`is_unquoted_char` filters out on-ASCII") }
    })
});

#[inline]
pub(crate) fn is_unquoted_char(c: u8) -> bool {
    matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_')
}

// dot-sep   = ws %x2E ws  ; . Period
const DOT_SEP: u8 = b'.';

#[cfg(test)]
mod test {
    use super::*;

    use combine::stream::position::Stream;
    use snapbox::assert_eq;

    #[test]
    fn keys() {
        let cases = [
            ("a", "a"),
            (r#""hello\n ""#, "hello\n "),
            (r#"'hello\n '"#, "hello\\n "),
        ];

        for (input, expected) in cases {
            let parsed = simple_key().easy_parse(Stream::new(input.as_bytes()));
            assert!(parsed.is_ok());
            let ((.., k), rest) = parsed.unwrap();
            assert_eq(k.as_str(), expected);
            assert_eq!(rest.input.len(), 0);
        }
    }
}