function rotate(translation::Int64, text::Char)
    translation %= 26
    if isletter(text)
        limit = isuppercase(text) ? 'Z' : 'z'
        text += translation
        if text > limit
            text -= 26
        end
    end
    return text
end
function rotate(translation::Int64, text::String)
    # ans = ""
    # for c in text
    #     ans *= rotate(translation, c)
    # end
    # return ans
    return map(c -> rotate(translation, c), text)
end

for i in 0:26
    macro_name = Symbol("R$(i)_str")
    @eval macro $macro_name(s)
        rotate($i, s)
    end
end