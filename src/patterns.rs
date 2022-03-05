use regex::Regex;

pub fn is_gacha(text: String) -> bool {
    let re = Regex::new(r"catgacha").unwrap();
    re.is_match(&text)
}
