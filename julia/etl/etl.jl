function transform(input::AbstractDict)
    # res = Dict()
    # for (value, letters) in input
    #     for letter in letters
    #         res[lowercase(letter)] = value
    #     end
    # end
    # return res
    return Dict(
        (lowercase(letter), value)
        for (value, letters) in input
        for letter in letters
    )
end

