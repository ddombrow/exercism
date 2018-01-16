pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for x in 0..limit {
       if factors.iter().fold(false, |test, curr| test || (x % curr == 0)) {
           sum += x
       }
    }

    sum
}

