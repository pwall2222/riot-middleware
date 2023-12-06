use once_cell::sync::Lazy;
use regex::bytes::Regex;

static OPEN: Lazy<Regex> = Lazy::new(|| {
    let reg = r#"<([a-zA-Z:_][a-zA-Z-_:.]*)(?:(?: |\n)*?[a-zA-Z:_][a-zA-Z-_:.]*=(?:\'|\")[^\'\"]*(?:\'|\"))*>"#;
    Regex::new(reg).unwrap()
});
static CLOSE: Lazy<Regex> = Lazy::new(|| Regex::new(r"</[a-zA-Z:_][a-zA-Z-_:.]* ?>").unwrap());
static STREAM: Lazy<Regex> = Lazy::new(|| Regex::new(r"stream:stream").unwrap());
static GREATER: Lazy<Regex> = Lazy::new(|| Regex::new(r">").unwrap());
static LESSER: Lazy<Regex> = Lazy::new(|| Regex::new(r"<").unwrap());

fn count(buff: &[u8], regex: &Lazy<Regex>) -> usize {
    regex.find_iter(buff).count()
}

pub(crate) fn check_xml(buff: &[u8]) -> bool {
    if count(buff, &GREATER) != count(buff, &LESSER) {
        return false;
    }
    let open_count = count(buff, &OPEN);
    let close_count = count(buff, &CLOSE);
    let stream_count = count(buff, &STREAM);
    if open_count == 0 && close_count == 0 && stream_count == 1 {
        return true;
    }
    if let Some(open_count) = open_count.checked_sub(stream_count) {
        if open_count == close_count {
            return true;
        }
    }
    false
}
