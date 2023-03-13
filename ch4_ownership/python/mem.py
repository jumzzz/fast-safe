import sys

l0 = [1,2,3,4,5]
print('ref_count = ', sys.getrefcount(l0))
l1 = l0
l2 = l0
print('ref_count = ', sys.getrefcount(l0))

