
def is_palindrome(i):
	return str(i) == str(i)[::-1]

def product_generator():
	largest_product = 0
	for i in xrange(999, 100, -1):
		for j in xrange(999, 100, -1):
			product = i * j
			if(is_palindrome(product) and product > largest_product):
				largest_product = product
				yield product
			else:
				continue

print(reduce(lambda acc, value: acc if acc > value else value, product_generator()))
