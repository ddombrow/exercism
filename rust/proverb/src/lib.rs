pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();

    for x in 0..list.len() {
        if x == (list.len() - 1) {
            proverb += format!("And all for the want of a {}.", list[0]).as_ref();
        }
        else {
            proverb += format!("For want of a {} the {} was lost.\n", list[x], list[x+1]).as_ref();
        }
    }
    
    proverb
}

