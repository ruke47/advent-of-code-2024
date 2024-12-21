2,4, bst A  -> b = a % 8
1,5, bxl 5  -> b = b ^ 101    -> b = (a % 8) ^ 5
7,5, cdv B  -> c = a / 2**b   -> c = a / 2 ** b    -> a / (2 ** (a % 8) ^ 5)
1,6, bxl 6  -> b = b ^ 110    -> b = ((a % 8) ^ 5) ^ 6  -> b = ((a % 8) ^ 3)
4,2, bxc 2  -> b = b ^ c      -> b = ((a % 8) ^ 3) ^ (a / (2 ** (a % 8) ^ 5))
5,5, out B  -> out = (((a % 8) ^ 3) ^ (a / (2 ** ((a % 8) ^ 5)))) % 8
0,3, adv 3  -> a = a / 8
3,0  jnz 0  -> if a > 0, loop

adv 3 -> jnz means shrinking a by 3 binary digits per loop
if a start less than 8, 1 loop
if a is 8..64, 2 loops
for 16 output, a should be in 8^(15)..8^(16)

the last output only cares about A % 8
