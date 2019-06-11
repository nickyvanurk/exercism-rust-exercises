pub fn nth(n: u32) -> u32 {  
    if n == 0 {
        return 2;
    }

    let mut count = 1;
    let mut num = 3;

    while count <= n {
        if is_prime(num) {
            if count == n {
                return num;
            }

            count += 1;
        }

        num += 2;
    }

    0
}

fn is_prime(n: u32) -> bool {
    for factor in 2..n-1 {
        if n % factor == 0 {
            return false;
        }
    }

    true
}
