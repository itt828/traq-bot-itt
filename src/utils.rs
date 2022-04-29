use crate::apis::channel::get_channels;

pub async fn get_uuid(path: &str) -> String {
    let mut cur_parent: Option<String> = None;
    let channels = get_channels().await;
    for node in path.split('/') {
        let node = node.replace("#", "");
        let p = channels
            .public
            .iter()
            .find(|x| x.name == node && x.parent_id == cur_parent)
            .unwrap()
            .id
            .clone();
        cur_parent = Some(p);
    }
    cur_parent.unwrap()
}
