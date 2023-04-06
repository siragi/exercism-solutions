pub const ComputationError = error{
    IllegalArgument,
};

pub fn steps(number: usize) anyerror!usize {
    // Take any positive integer n.
    if (number < 1) return ComputationError.IllegalArgument;
    return collatz(number);
}

fn collatz(number: usize) usize {
    // Base case to stop recursion
    if (number == 1) return 0;

    // Repeat the process indefinitely. (Here: Recursion)
    // The conjecture states that no matter which number you start with, you will always reach 1 eventually.

    if (number & 1 == 0) {
        // If n is even, divide n by 2 to get n / 2.
        return 1 + collatz(number >> 1);
    } else {
        // If n is odd, multiply n by 3 and add 1 to get 3n + 1.
        return 1 + collatz(number * 3 + 1);
    }
}
