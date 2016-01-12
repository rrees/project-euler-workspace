
function palindrome(i)
	return string(i) == reverse(string(i))
end

products = [i*j for i=999:-1:100, j=999:-1:100]


total = maximum(filter(palindrome, products))

println(total)
