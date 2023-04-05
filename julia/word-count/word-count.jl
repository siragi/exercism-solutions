function wordcount(sentence::String)
    # a) using regex eachmatch
    #word_regex = r"[[:alnum:]]+('[[:alnum:]]+)*"
    #words = [m.match for m in eachmatch(word_regex, lowercase(sentence))]

    # b) using pipes feeding lambdafunctions (anonymous fn)->
    words = (lowercase(sentence)
            |> lowered -> replace(lowered, ',' => " ")
            |> replaced -> filter(c -> isdigit(c) || isletter(c) || c == '\'' || isspace(c), replaced)
            |> split
            |> words -> map(word -> strip(word, '\''), words)
    )

    words == [""] && return Dict()   # shortform for if no valid input return empty dict immediately


    # Dict(word => length(filter(w -> w == word, words))
    #      for word in unique(words))
    Dict(word => count(==(word), words)  # no need to wrap equal function '==()' into an anonymous function
        for word in unique(words))
end