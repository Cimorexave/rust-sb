pub(crate) mod fibo {
    pub fn fib(n: u32) -> u64 {
         if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            return fib(n - 1) + fib(n - 2);
        }
    }
}
