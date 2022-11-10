use regex::Regex;

pub fn is_gacha(text: &str) -> bool {
    let re = Regex::new(r"catgacha").unwrap();
    if re.is_match(text) {
        return true;
    }
    false
}
pub fn is_cutgacha(text: &str) -> bool {
    let re = Regex::new(r"cutgacha").unwrap();
    if re.is_match(text) {
        return true;
    }
    false
}
pub fn is_itt(text: &str) -> bool {
    let re = Regex::new(r"(?i)itt").unwrap();
    if re.is_match(text) {
        return true;
    }
    false
}
pub fn extract_message_id(url: &str) -> Option<String> {
    let re =
        Regex::new(r"^(https://q.trap.jp/messages/)?(?P<message_id>[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})$").unwrap();
    if let Some(matches) = re.captures(url) {
        let message_id = &matches["message_id"];
        return Some(message_id.to_string());
    }
    None
}

#[test]
fn extract_message_id_works() {
    let url = r"https://q.trap.jp/messages/33a865ef-cae1-4f79-a65f-a02eb9d7ca48";
    let s = extract_message_id(url);
    assert_eq!(
        s,
        Some(String::from("33a865ef-cae1-4f79-a65f-a02eb9d7ca48"))
    );
}
#[test]
fn extract_message_id_works2() {
    let url = r"33a865ef-cae1-4f79-a65f-a02eb9d7ca48";
    let s = extract_message_id(url);
    assert_eq!(
        s,
        Some(String::from("33a865ef-cae1-4f79-a65f-a02eb9d7ca48"))
    );
}
