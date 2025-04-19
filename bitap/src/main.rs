fn main () {
    bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "2JZ");
    bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "RB26DETT");
    bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "");
}

fn bitap_search (texto: &str, padrao: &str) {
    let m: usize = padrao.chars().count();
    let mut i: usize = 0;
    let mut r: Vec<bool> = vec![false; m+1];
    r[0 as usize] = true;

    if m == 0 {
        println!("Padrao vazio...");
        return;
    }
    
    while i < texto.chars().count() {
        let mut k: usize = m;
        while k >=1 {
            if r[k-1] && texto.chars().nth(i).unwrap() == padrao.chars().nth(k-1).unwrap() {
                r[k] = true;
            } else{
                r[k] = false;
            }
            k -= 1;
        }
        if r[m] {
            let index_location: usize = i-m+1;
            println!("Ha uma palavra neste index:  {index_location}");
            return;
        }
        i += 1;

    }
    println!("Nao foi encontrada uma palavra igual");
    return;
}
