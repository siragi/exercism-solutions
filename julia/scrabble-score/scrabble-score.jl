const scrabblescores = zip(
    ["AEIOULNRST", "DG", "BCMP", "FHVWY", "K", "JX", "QZ"],
    vcat(1:5,8,10))
const scores = Dict([char => score 
              for (letters, score) in scrabblescores  
              for char in letters])

function score(str)
    # 1)
    # ans = 0
    # for char in uppercase(str), (letters, score) in scrabblescores
    #     if char in letters
    #         ans += score
    #     end        
    # end
    # return ans

    # 2a) sort of switchcasing in inner function
    # f(c) = c in "AEIOULNRST" ? 1 :
    #        c in "DG" ? 2 :
    #        c in "BCMP" ? 3 :
    #        c in "FHVWY" ? 4 :
    #        c == 'K' ? 5 : 
    #        c in "JX" ? 8 :
    #        c in "QZ" ? 10 :
    #        0
    # sum([f(c) for c in uppercase(str)])

    # 2b) concise but looking through letters every time (if clause)
    # sum([score for (letters, score) in scrabblescores for char in uppercase(str) if char in letters])

    # 3) using the List comprehension for setting up Dict and using it directly
    sum([get(scores, char, 0) for char in uppercase(str)]; init=0)
end