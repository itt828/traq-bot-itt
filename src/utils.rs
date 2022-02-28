fn remove_double_quotes(string: String) -> String {
    let le = string.len();
    string[1..le - 1].to_string()
}
