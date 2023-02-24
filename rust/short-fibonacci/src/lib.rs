pub fn create_empty() -> Vec<usize> {
    Vec::new()
}

pub fn create_buffer(count: usize) -> Vec<usize> {
    vec![0; count]
}

// The "real" way
pub fn fibonacci() -> Vec<usize> {
    actual_fib(5)
}

pub fn actual_fib(count: usize) -> Vec<usize> {
    match count {
        0 => return create_empty(),
        1 => return vec![1],
        _ => {
            let mut buff = create_buffer(count);
            buff[0] = 1; buff[1] = 1;
            for ind in 2..count {
                buff[ind] = buff[ind-1] + buff[ind-2];
            }
            return buff
        }
    }
}