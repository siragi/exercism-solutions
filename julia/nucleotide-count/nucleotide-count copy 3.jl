"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
	isempty(strand) && return Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
	
    # ACGT = reduce(+, map(quantify, collect(strand)))
    # in its optimized form:
	ACGT::UInt64 = mapreduce(quantify, +, collect(strand))
    A = ACGT & 0xffff 
    C = ACGT >> 16 & 0xffff
    G = ACGT >> 32 & 0xffff
    T = ACGT >> 48

    return Dict('A' => A, 'C' => C, 'G' => G, 'T' => T)
end

function quantify(char::Char)::UInt64  
    char == 'A' ? 1 :
    char == 'C' ? 1 << 16 :
    char == 'G' ? 1 << 32 :
    char == 'T' ? 1 << 48 :
    throw(DomainError(char , " is no nucleotide"))
end
