### A Pluto.jl notebook ###
# v0.19.42

using Markdown
using InteractiveUtils

# ╔═╡ 285d542d-3695-433f-a1ae-600653ea3adf
begin
	strand = "GATTACA"
	emptystrand = ""
	invalidstrand = "José"
end

# ╔═╡ 4d5b8555-2ae0-4562-a9b5-61025a46d83e
function quantify(char) 
		local acc = [0,0,0,0]
		char == 'A' ? acc[1] += 1 :
		char ==    'C' ? acc[2] += 1 :
		char ==		'G' ? acc[3] += 1 :
		char ==		'T' ? acc[4] += 1 :
				throw(DomainError(char , " is no nucleotide"))
		return acc
end

# ╔═╡ ecec78b5-bce4-4ef2-900e-567b37c816b7
zipped = zip(["A", "C", "G", "T"], [0,0,0,0] )


# ╔═╡ 5b4be37d-5db6-42ca-bbc3-36eaee6e8209
quantify('G')



# ╔═╡ b0c8f5ff-8153-48e2-abe7-8744e643c051
count_nucleotides(strand)

# ╔═╡ 512b10ea-08be-41f0-ad6b-5884e464fbce
count_nucleotides(emptystrand)

# ╔═╡ c8649712-344b-45e8-8829-b7d9af83511f
isa(count_nucleotides(invalidstrand), DomainError)

# ╔═╡ 8bf53af0-2321-11ef-394b-2526c3f005bb
# ╠═╡ disabled = true
#=╠═╡
"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
	dict = Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
	isempty(strand) && return dict
	
	for byte in strand
		if byte in keys(dict)
			m = get(dict, byte, 0)
			push!(dict, byte => m + 1)
		else
            throw(DomainError(byte," in strand is no nucleotides"))
			
		end
	end

	return dict
end
  ╠═╡ =#

# ╔═╡ 5374bc57-2671-4070-b70a-239fa4eeacb4
# ╠═╡ disabled = true
#=╠═╡
"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
	# dict = Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
	dict = Dict(["A", "C", "G", "T"], [0,0,0,0])
	isempty(strand) && return dict

	

	
		
	
	acc = mapreduce(f, +, strand, [0,0,0,0])
	
	dict = Dict([])
    
			
	

	return dict
end
  ╠═╡ =#

# ╔═╡ Cell order:
# ╠═285d542d-3695-433f-a1ae-600653ea3adf
# ╠═8bf53af0-2321-11ef-394b-2526c3f005bb
# ╠═5374bc57-2671-4070-b70a-239fa4eeacb4
# ╠═4d5b8555-2ae0-4562-a9b5-61025a46d83e
# ╠═ecec78b5-bce4-4ef2-900e-567b37c816b7
# ╠═5b4be37d-5db6-42ca-bbc3-36eaee6e8209
# ╠═b0c8f5ff-8153-48e2-abe7-8744e643c051
# ╠═512b10ea-08be-41f0-ad6b-5884e464fbce
# ╠═c8649712-344b-45e8-8829-b7d9af83511f
