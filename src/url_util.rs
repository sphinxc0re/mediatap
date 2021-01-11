pub fn expand_to_full_url(base_url: &str, diff_str: &str) -> String {
    let mut split: Vec<_> = diff_str.split('|').collect();

    let split_position: usize = split.remove(0).parse().unwrap();
    let replacement_str = split.remove(0);

    let mut res = base_url.to_string();

    res.replace_range(split_position.., replacement_str);

    res
}
