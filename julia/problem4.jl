
function palindrome(i)
	return string(i) == reverse(string(i))
end

function find_largest_palindrome()
	largest_palindrome = 0
	for i in 100:999
		for j in 100:999
			product = i * j
			if (product > largest_palindrome) && palindrome(product)
				largest_palindrome = product
			end
		end
	end

	return largest_palindrome
end

total = find_largest_palindrome()

println(total)
