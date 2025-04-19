def Bitap_search (texto, padrao):
    m = len(list(padrao))
    r = [0]*(m+1)
    r[0] = 1
    i = 0

    if m == 0:
        print("Padrão vazio...")
        return
    while i < len(list(texto)):
        k = m
        while k >= 1:
            if r[k-1] and list(texto)[i] == list(padrao)[k-1]:
                r[k] = True
            else:
                r[k] = False
            k -= 1
        if r[m]:
           print('Há esta palavra neste index: ', (i - m) + 1)
           return
        i += 1
    print("Não foi encontrada uma palavra igual no texto")
    return

Bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "2JZ")
Bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "RB26DETT")
Bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "")