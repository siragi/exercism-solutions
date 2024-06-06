"""
count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
## 1st version (the oh so lame for loop being just fast!)
# function count_nucleotides(strand)
# 	# dict = Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
# 	#  or smooth like this
	# basecount = Dict(base => 0 for base in "ACGT")

# 	for byte in strand
# 		if byte in keys(basecount)
# 			basecount[byte] += 1
# 			# m = get(dict, byte, 0)
# 			# push!(dict, byte => m + 1)
# 		else
#             throw(DomainError(byte," in strand is no nucleotides"))

# 		end
# 	end

## using try and catch instead of checking keys first
function count_nucleotides(strand)
	basecount = Dict(base => 0 for base in "ACGT")

	for byte in strand
		try
			basecount[byte] += 1
        catch e # exception maybe following Error: KeyError(key): key '_' not found (https://docs.julialang.org/en/v1/base/base/#Base.KeyError)
            e == KeyError(byte) ? throw(DomainError(byte,"$byte in strand is no nucleotide")) :
            throw(e)
		end
	end

	return basecount
end

## 1b) same principle, but using list comprehension (which produces an unused list, therefore slower)
# function count_nucleotides(strand)
#     result = Dict(base => 0 for base in "ACGT")
#     [byte in keys(result) ? result[byte] += 1 : throw(DomainError(byte)) for byte in strand]
#     return result
# end

## 1c) very elegant
# function count_nucleotides(strand) 
#     # if strand issubset or equal "ACGT" (\subseteq) => no nonsense if true 
#     if strand ⊆ "ACGT" 
#         Dict(base => count(base, strand) for base in "ACGT") #  uses existing count (every base in strand) function
#     else 
#         throw(DomainError(strand))
#     end
# end
## 2nd version (cooler, but 3 times slower)
# function count_nucleotides(strand)
# 	isempty(strand) && return Dict(base => 0 for base in "ACGT")

#     basecount = sum(map(quantify, collect(strand)))
#     # = reduce(+, map(quantify, collect(strand)))
#     # in its optimized form:
# 	# = mapreduce(quantify, +, collect(strand))
    
# 	return Dict(zip(collect("ACGT"), basecount))
# end

# function quantify(char::Char) 
#     char == 'A' ? [1,0,0,0] :
#     char == 'C' ? [0,1,0,0] :
#     char == 'G' ? [0,0,1,0] :
#     char == 'T' ? [0,0,0,1] :
#     throw(DomainError(char , " is no nucleotide"))
# end

## 3rd version using a bitmap instead of a basecount-list 
## (but without boundery overflow check and not faster than first version)
# function count_nucleotides(strand)
# 	isempty(strand) && return Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
	
# 	ACGT::UInt64 = mapreduce(quantify, +, collect(strand))
#     A = ACGT & 0xffff 
#     C = ACGT >> 16 & 0xffff
#     G = ACGT >> 32 & 0xffff
#     T = ACGT >> 48

#     return Dict('A' => A, 'C' => C, 'G' => G, 'T' => T)
# end

# function quantify(char::Char)::UInt64  
#     char == 'A' ? 1 :
#     char == 'C' ? 1 << 16 :
#     char == 'G' ? 1 << 32 :
#     char == 'T' ? 1 << 48 :
#     throw(DomainError(char , " is no nucleotide"))
# end
