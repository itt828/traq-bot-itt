use regex::Regex;

pub fn is_gacha(text: String) -> bool {
    let re = Regex::new(r"catgacha").unwrap();
    if re.is_match(&text) {
        println!("is_gacha matched!");
        return true;
    }
    false
}
