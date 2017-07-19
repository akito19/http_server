use std::str::from_utf8;

pub enum ParseResult<T> {
    Complete(T),
    Partial,
    Error,
}

impl<T> ParseResult<T> {
    fn is_complete(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Complete(_) => true,
            _ => false
        }
    }

    fn is_partial(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Partial => true,
            _ => false
        }
    }
}

impl<T, E> From<Result<T, E>> for ParseResult<T> {
    fn from(r: Result<T, E>) -> Self {
        use self::ParseResult::*;
        match r {
            Ok(r) => Complete(r),
            Err(_) => Error,
        }
    }
}

pub struct Request<'a>(pub &'a str);

pub fn parse(mut buf: &[u8]) -> ParseResult<Request> {
    use self::ParseResult::*;

    let get = b"GET ";
    let end = b"\r\n";
    if !buf.starts_with(get) {
        return Error;
    }

    buf = &buf[get.len()..];
    if buf.ends_with(end) {
        buf = &buf[0..buf.len() - end.len()]
    } else {
        return Partial;
    }

    from_utf8(buf)
        .map(Request)
        .into()
}

#[test]
fn http09_get_success_root() {
    let req = b"GET /\r\n";
    let res = parse(req);
    assert!(res.is_complete());
}

#[test]
fn http09_get_success_foo_bar() {
    let req = b"GET /foo/bar\r\n";
    let res = parse(req);
    assert!(res.is_complete());
}

#[test]
fn http09_get_partial_root() {
    let req = b"GET /\r";
    let res = parse(req);
    assert!(res.is_partial());
}

#[test]
#[should_panic]
fn http09_post_failure() {
    let req = b"POST /\r\n";
    let res = parse(req);
    assert!(res.is_complete());
}
