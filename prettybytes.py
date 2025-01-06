def pretty(b: list[int]) -> str:
    out = []
    for c in b:
        char = chr(c)
        if char.isalpha():
            out.append(char)
        else:
            out.append("["+ str(c)+ "]")
    return "".join(out) 

