pub const ComputationError = error{
    IllegalArgument,
};

pub fn steps(number: usize) anyerror!usize {
    // Take any positive integer n.
    if (number < 1) {
        return ComputationError.IllegalArgument;
    }

    var n: usize = number;
    var cnt: usize = 0;

    // Repeat the process indefinitely.
    // The conjecture states that no matter which number you start with, you will always reach 1 eventually.
    while (n > 1) : (cnt += 1) { // while condition : 'continue expression'
        if (((n >> 1) << 1) == n) {
            n = n >> 1; // If n is even, divide n by 2 to get n / 2.
        } else {
            n = n * 3 + 1; // If n is odd, multiply n by 3 and add 1 to get 3n + 1.
        }
    }

    return cnt;
}
