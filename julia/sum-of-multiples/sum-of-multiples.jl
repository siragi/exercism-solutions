function sum_of_multiples(limit, factors)
    isempty(factors) && return 0

    sum(unique([f * n
                for f in factors
                for n in 1:limit if f * n < limit
    ]))
end
