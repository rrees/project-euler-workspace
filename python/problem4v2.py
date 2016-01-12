
def is_palindrome(i):
	return str(i) == str(i)[::-1]

comprehension = [product for product in [i * j for j in xrange(999, 100, -1) for i in xrange(999, 100, -1)] if is_palindrome(product)]


print(max(comprehension))
