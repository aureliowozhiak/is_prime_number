mod is_prime_number{

    pub fn is_prime_number(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

}