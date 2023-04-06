pub const ComputationError = error{IllegalArgument};

pub fn steps(number: usize) anyerror!usize {
    // Take any positive integer n.
    if (number < 1) return ComputationError.IllegalArgument;
    // Base case to stop recursion
    if (number == 1) return 0;

    // Repeat the process indefinitely. (Here: Recursion)
    // The conjecture states that no matter which number you start with, you will always reach 1 eventually.

    if (number & 1 == 0) {
        // If n is even, divide n by 2 to get n / 2.
        return 1 + (steps(number >> 1) catch unreachable); // catch: if problem take rhs as default
        // unreachable: i hereby state im am unreachable (catching an error is not going to happen),
        //              if i am wrong though: panic (in Debug and ReleaseSafe compile mode)
        // source: https://ziglang.org/documentation/master/#catch
    } else {
        // If n is odd, multiply n by 3 and add 1 to get 3n + 1.
        return 1 + (steps(number * 3 + 1) catch unreachable);
    }
}
