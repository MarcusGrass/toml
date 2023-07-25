use crate::InternalString;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Opaque string storage for raw TOML; internal to `toml_edit`
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct RawString(RawStringInner);

#[derive(PartialEq, Eq, Clone, Hash)]
enum RawStringInner {
    Empty,
    Explicit(InternalString),
    Spanned(core::ops::Range<usize>),
}

impl RawString {
    pub(crate) fn with_span(span: core::ops::Range<usize>) -> Self {
        if span.start == span.end {
            RawString(RawStringInner::Empty)
        } else {
            RawString(RawStringInner::Spanned(span))
        }
    }

    /// Access the underlying string
    pub fn as_str(&self) -> Option<&str> {
        match &self.0 {
            RawStringInner::Empty => Some(""),
            RawStringInner::Explicit(s) => Some(s.as_str()),
            RawStringInner::Spanned(_) => None,
        }
    }

    pub(crate) fn to_str<'s>(&'s self, input: &'s str) -> &'s str {
        match &self.0 {
            RawStringInner::Empty => "",
            RawStringInner::Explicit(s) => s.as_str(),
            RawStringInner::Spanned(span) => input.get(span.clone()).unwrap_or_else(|| {
                panic!("span {:?} should be in input:\n```\n{}\n```", span, input)
            }),
        }
    }

    pub(crate) fn to_str_with_default<'s>(
        &'s self,
        input: Option<&'s str>,
        default: &'s str,
    ) -> &'s str {
        match &self.0 {
            RawStringInner::Empty => "",
            RawStringInner::Explicit(s) => s.as_str(),
            RawStringInner::Spanned(span) => {
                if let Some(input) = input {
                    input.get(span.clone()).unwrap_or_else(|| {
                        panic!("span {:?} should be in input:\n```\n{}\n```", span, input)
                    })
                } else {
                    default
                }
            }
        }
    }

    /// Access the underlying span
    pub(crate) fn span(&self) -> Option<core::ops::Range<usize>> {
        match &self.0 {
            RawStringInner::Empty => None,
            RawStringInner::Explicit(_) => None,
            RawStringInner::Spanned(span) => Some(span.clone()),
        }
    }

    pub(crate) fn despan(&mut self, input: &str) {
        match &self.0 {
            RawStringInner::Empty => {}
            RawStringInner::Explicit(_) => {}
            RawStringInner::Spanned(span) => {
                *self = Self::from(input.get(span.clone()).unwrap_or_else(|| {
                    panic!("span {:?} should be in input:\n```\n{}\n```", span, input)
                }))
            }
        }
    }

    pub(crate) fn encode(&self, buf: &mut dyn core::fmt::Write, input: &str) -> core::fmt::Result {
        let raw = self.to_str(input);
        for part in raw.split('\r') {
            write!(buf, "{}", part)?;
        }
        Ok(())
    }

    pub(crate) fn encode_with_default(
        &self,
        buf: &mut dyn core::fmt::Write,
        input: Option<&str>,
        default: &str,
    ) -> core::fmt::Result {
        let raw = self.to_str_with_default(input, default);
        for part in raw.split('\r') {
            write!(buf, "{}", part)?;
        }
        Ok(())
    }
}

impl Default for RawString {
    fn default() -> Self {
        Self(RawStringInner::Empty)
    }
}

impl core::fmt::Debug for RawString {
    #[inline]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match &self.0 {
            RawStringInner::Empty => write!(formatter, "empty"),
            RawStringInner::Explicit(s) => write!(formatter, "{:?}", s),
            RawStringInner::Spanned(s) => write!(formatter, "{:?}", s),
        }
    }
}

impl From<&str> for RawString {
    #[inline]
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<String> for RawString {
    #[inline]
    fn from(s: String) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<&String> for RawString {
    #[inline]
    fn from(s: &String) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<InternalString> for RawString {
    #[inline]
    fn from(inner: InternalString) -> Self {
        Self(RawStringInner::Explicit(inner))
    }
}

impl From<&InternalString> for RawString {
    #[inline]
    fn from(s: &InternalString) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<Box<str>> for RawString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}
