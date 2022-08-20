#[derive(Debug)]
pub struct Ical {
    url: String,
    title: Option<String>,
    notify_time: Vec<usize>,
}
