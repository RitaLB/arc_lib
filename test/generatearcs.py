arcs = []
n = 11
for l in range (n):
    h = l+1
    while h != n:
        arcs.append((l,h))
        h = h+1

print(arcs)
print(len(arcs))