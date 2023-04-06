function largest_product(str, span)
    num_windows = length(str) - span + 1
    if num_windows < 1 || span < 0
        throw(ArgumentError("Span needs to be between 0 and length(str)"))
    end
    digit_list = [parse(Int64, c) for c in str]
    @views products = [
        prod(digit_list[i:i+span-1])
        for i in 1:num_windows
    ]
    return maximum(products)
end