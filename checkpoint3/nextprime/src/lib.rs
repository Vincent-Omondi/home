pub fn next_prime(nbr: u64) -> u64 {
    if is_prime(nbr){
        return nbr
    } 
    let mut nxt = nbr + 1;
    loop {
        if is_prime(nxt) {
            return nxt
        }
        nxt += 1
    }
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false
    }
    for i in 2..n {
        if n%i == 0 {
            return false
        }
    }
    true
}