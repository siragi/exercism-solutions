pub fn squareOfSum(number: usize) usize {
    // computes the sum of i's from 0 to n and squares it
    var sum: usize = 0;
    // works with zig version 0.11.0-dev.2375+771d07268
    // for (1..number + 1) |i| {
    //     sum += i;
    // }

    var i: usize = number;
    while (i > 0) : (i -= 1) {
        sum += i;
    }
    return sum * sum;
}

pub fn sumOfSquares(number: usize) usize {
    // computes the sum of i^2 from 0 to n
    var sum: usize = 0;
    // works with zig version 0.11.0-dev.2375+771d07268
    // for (1..number + 1) |i| {
    //     sum += i * i;
    // }
    var i: usize = number;
    while (i > 0) : (i -= 1) {
        sum += i * i;
    }
    return sum;
}

pub fn differenceOfSquares(number: usize) usize {
    // computes the difference between the square of sum and sum of squares;
    return squareOfSum(number) - sumOfSquares(number);
}
