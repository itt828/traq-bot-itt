use regex::Regex;

pub fn is_gacha(text: String) -> bool {
    let re = Regex::new(r"kinanogacha").unwrap();
    re.is_match(&text)
}
