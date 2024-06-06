"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
	isempty(strand) && return Dict(base => 0 for base in "ACGT")
	
    # r = reduce(+, map(quantify, collect(strand)))
    # in its optimized form:
	r = mapreduce(quantify, +, collect(strand))
    
	return Dict(zip(collect("ACGT"),r))
end

function quantify(char::Char) 
    char == 'A' ? [1,0,0,0] :
    char == 'C' ? [0,1,0,0] :
    char == 'G' ? [0,0,1,0] :
    char == 'T' ? [0,0,0,1] :
    throw(DomainError(char , " is no nucleotide"))
end