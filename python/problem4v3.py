
def is_palindrome(i):
	return str(i) == str(i)[::-1]

def product_generator():
	for i in xrange(999, 100, -1):
		for j in xrange(999, 100, -1):
			product = i * j
			if(is_palindrome(product)):
				yield product
			else:
				continue

print(reduce(lambda acc, value: acc if acc > value else value, product_generator()))
