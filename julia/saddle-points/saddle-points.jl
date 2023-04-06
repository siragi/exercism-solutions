function saddlepoints(M)
    minpercol = minimum(M, dims=1, init=Inf) # minimum represented in dimension 1 (as a row) , 
    # init (assume the small number of infinity if no other minimum is found:-) ), thus eliminating the need of checking for empty M

    maxperrow = maximum(M, dims=2, init=-Inf) # maximum represented in dimension 2 (as a vector)

    return [(row, col)
            for row in axes(M, 1), col in axes(M, 2)
            if M[row, col] == maxperrow[row] == minpercol[col]] # creates empty list if no min or max can be found
end
