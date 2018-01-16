pub fn reverse(target: &str) -> String {
    let mut letters = target.chars();
    let mut reversed = String::from("");

    for _x in 0..target.len() {
        reversed.push(letters.next_back().unwrap());
    }

    reversed
}
