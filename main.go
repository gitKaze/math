package main

import (
	f "fmt"
	"math/rand"
	"strconv"
	"time"

	r "github.com/gitKaze/math/real"
)

const testAmount = 10

func test(v1, v2 string) {
	var x1, x2, result r.BigInt
	x1.NewInt(v1)
	x2.NewInt(v2)
	f.Printf("\nSTART\n")
	f.Printf("N1:  %v\nN2:  %v\n", x1, x2)
	start := time.Now()
	result.Sub(x1, x2)
	end := time.Since(start)
	f.Printf("SUB in: %v\n%v\n", end, result)
	start = time.Now()
	result.Add(x1, x2)
	end = time.Since(start)
	f.Printf("ADD in: %v\n%v\n", end, result)
	start = time.Now()
	result.MulAbs(x1, x2)
	end = time.Since(start)
	f.Printf("MUL in: %v\n%v\n", end, result)
	f.Printf("END\n")
}
func testWmetric(v1, v2 string) {
	var x1, x2, result r.BigInt
	var lenghts [3]int
	f.Printf("%v | %v ", v1, v2)
	start := time.Now()
	x1.NewInt(v1)
	x2.NewInt(v2)
	end := time.Since(start)
	f.Printf("initialized in %v\n", end)
	f.Printf("lenght: %v | %v (base:2^64)\n", len(x1.Body), len(x2.Body))
	start = time.Now()
	result.Sub(x1, x2)
	end = time.Since(start)
	lenghts[0] = len(result.Body)
	f.Printf("SUB completed in: %v\n", end)
	start = time.Now()
	result.Add(x1, x2)
	end = time.Since(start)
	lenghts[1] = len(result.Body)
	f.Printf("ADD completed in: %v\n", end)
	start = time.Now()
	result.MulAbs(x1, x2)
	end = time.Since(start)
	lenghts[2] = len(result.Body)
	f.Printf("MUL completed in: %v\n", end)
	f.Printf("result lenghts: %v\n", lenghts)
	f.Printf("END\n")
}
func main() {
	DoTest := true
	testWmetric("1e10000", "2^57898")
	for i := 1; i <= testAmount && DoTest; i++ {
		a, b, x, y := rand.Intn(500000), rand.Intn(100000), rand.Intn(500000), rand.Intn(100000)
		strs := []string{strconv.Itoa(a) + "^" + strconv.Itoa(b), strconv.Itoa(x) + "^" + strconv.Itoa(y)}
		testWmetric(strs[0], strs[1])
	}
}
