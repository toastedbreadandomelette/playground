def fac(n):
    return 1 if n <= 1 else n*fac(n-1)

# a = fac(21)//(fac(4)*fac(5)*fac(4)*fac(8))
# b = fac(20)//(fac(3)*fac(5)*fac(4)*fac(8)) + fac(20)//(fac(4)*fac(4)*fac(4)*fac(8)) + fac(20)//(fac(4)*fac(5)*fac(3)*fac(8)) + fac(20)//(fac(4)*fac(5)*fac(4)*fac(7))
# print(a, b, a==b)

a = fac(4) - fac(4)//fac(1) + fac(4)//fac(2) - fac(4)//fac(3) + 1
print(a)