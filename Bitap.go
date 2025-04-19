package main

import (
	"fmt"
)

func Bitap_search(texto string, padrao string) {
	var m int = len(padrao)
	var i int = 0
	r := make([]int, m+1)
	r[0] = 1

	if m == 0 {
		fmt.Printf("Padrão vazio...")
		return
	}

	for i < len(texto) {
		var k int = m
		for k >= 1 {
			if r[k-1] == 1 && texto[i] == padrao[k-1] {
				r[k] = 1
			} else {
				r[k] = 0
			}
			k -= 1
		}
		if r[m] == 1 {
			fmt.Println("Há esta palavra neste index:", (i-m)+1)
			return
		}
		i += 1
	}
	fmt.Println("Não foi encontrada uma palavra igual no texto")
	return
}

func main() {
	Bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "2JZ")
	Bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "RB26DETT")
	Bitap_search("Toyota Supra MK4 with 2JZ-GTE 3.0L Inline-6 Turbocharged Engine and RWD", "")
}
