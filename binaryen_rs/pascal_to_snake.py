import re

with open('binops_macro.txt') as f:
    for line in f:
        source = line.strip()
        name = re.sub(r'(?<!^)(?=[A-Z])', '_', source).lower()
        # print(name[1:]) # 1: to remove a `_` from the start
        #    binop!(neg_float_int_32, BinaryenNegFloat32);
        print(f"binop!({name}, Binaryen{source});")