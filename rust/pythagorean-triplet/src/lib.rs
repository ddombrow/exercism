pub fn find() -> Option<u32> {
    let mut x = 0_u32; 
    for a in 1..1000 {
        for b in a..1000 {
            for c in b..1000 {
                if (a as u32).pow(2) + (b as u32).pow(2) == (c as u32).pow(2) {
                    if (a+b+c) == 1000 {
                       x = a * b * c;
                       break;
                    }
                }
            }
        }
    }

    Some(x)
}
