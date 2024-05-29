pub fn response(s: []const u8) []const u8 {
    var qulo: u4 = 0b0000; // question, upper, lower, other
    for (s) |byte| qulo = switch (byte) {
        '\t'...' ' => continue, // includes ascii 0x09 (horizontal tab \t), 0x10 (linefeed or newline \n), 0x0D(carriage return \r) to 0x20 (space)
        '?' => qulo | 0b1000, // sets q-bit
        'A'...'Z' => qulo & 0b0111 | 0b0100, // sets q-bit off first (since ? obv is not at last pos) then sets u-bit
        'a'...'z' => qulo & 0b0111 | 0b0010, // sets q-bit off then sets l-bit
        else => qulo & 0b0111 | 0b0001, // q-bit off than o-bit on
    };
    // bitmask(& 0bxxxx) sets irrelevant bits off, so comparison has not to deal with multiple possible result 
    // (btw: formatter will change following to a oneliner)
    return if (qulo & 0b1110 == 0b1100) "Calm down, I know what I'm doing!" 
      else if (qulo & 0b1000 == 0b1000) "Sure." 
      else if (qulo & 0b0110 == 0b0100) "Whoa, chill out!" 
      else if (qulo == 0b0000) "Fine. Be that way!" 
      else "Whatever.";

    // var ans: []const u8 = undefined;
    // if (qulo & 0b1110 == 0b1100) ans = "Calm down, I know what I'm doing!" else {
    //     if (qulo & 0b1000 == 0b1000) ans = "Sure." else {
    //         if (qulo & 0b0110 == 0b0100) ans = "Whoa, chill out!" else {
    //             if (qulo == 0b0000) ans = "Fine. Be that way!" else ans = "Whatever.";
    //         }
    //     }
    // }

    // return ans;
}
