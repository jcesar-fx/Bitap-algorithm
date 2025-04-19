function bitapSearch(texto, padrao){
    const m = padrao.length;
    let i = 0;
    let r = new Array(m+1).fill(0);
    r[0] = 1;
    
    if (m == 0){
        console.log("Padrão vazio...");
        return;
    };

    while (i < texto.length){
        let k = m;
        while (k >= 1){
            if (r[k-1] && texto[i] == padrao[k-1]){
                r[k] = true;
            }
            else{
                r[k] = false;
            };
            k -= 1;
        };
        if (r[m]){
            console.log('Há uma palavra neste index: ', (i - m) + 1);
            return;
        };
        i += 1
    };
    console.log("Não foi encontrada uma palavra igual")
    return

};

bitapSearch("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "2JZ");
bitapSearch("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "RB26DETT");
bitapSearch("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "");