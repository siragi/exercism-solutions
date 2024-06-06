"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
	# dict = Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
	#  or smooth like this
	basecount = Dict(base => 0 for base in "ACGT")
	
	for byte in strand
		if byte in keys(basecount)
			basecount[byte] += 1
			# m = get(dict, byte, 0)
			# push!(dict, byte => m + 1)
		else
            throw(DomainError(byte," in strand is no nucleotides"))
			
		end
	end

	return basecount
end