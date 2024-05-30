# 00001 = wink
# 00010 = double blink
# 00100 = close your eyes
# 01000 = jump
# 10000 = Reverse the order of the operations in the secret handshake.

const actions = ["wink", "double blink", "close your eyes", "jump"]

function secret_handshake(code::Integer)
    bitset = digits(code, base=2, pad=5)
    ans =  [act for (i, act) in enumerate(actions) if bitset[i] == 1]
    return bitset[5] == 0 ? ans : reverse(ans)
        
    # 3) clumsy but it works in one go.
    # bin = digits(code, base=2, pad=5)
    # offset = bin[5] == 0 ? 0 : 5  # offset enables reverse fabrication of actions:
    # return [actions[abs(offset-i)] for i in eachindex(actions) if bin[abs(offset-i)] == 1]

    # 2) Less handcoding, 
    # ans = []
    # for act in actions
    #     code & 0b1 == 0b1 && push!(ans, act) # comparing only last digit
    #     code >>= 1 # discard last bit by shifting it out
    # end
    # return code & 0b1 == 0b1 ? reverse(ans) : ans

    # 1)
    # code & 0b1 == 0b1 && push!(ans, "wink") 
    # code & 0b10 == 0b10 && push!(ans, "double blink")
    # code & 0b100 == 0b100 && push!(ans, "close your eyes")
    # code & 0b1000 == 0b1000 && push!(ans, "jump")
    # code & 0b10000 == 0b10000 && reverse!(ans)
    # return ans
end
