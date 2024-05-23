n = int(input())
linhas = (2**n)
#print(linhas)
resultado = 0
for i in range(1, linhas):
    #print(resultado)
    #print("sum =", linhas -i)
    resultado = resultado + (linhas - i)

print(resultado)