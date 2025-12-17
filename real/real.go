package real

import (
	"math/bits"
	"strconv"
	"strings"
)

const word = 18
const maxuint64 = 18446744073709551615

type BigInt struct {
	Neg  bool
	Body []uint64
}

func parse(str string) (result []string, variant int8) {
	result = strings.Split(str, "e")
	if len(result) == 2 {
		return result, 3
	}
	result = strings.Split(str, "^")
	if len(result) == 2 {
		return result, 2
	}
	if len(result) == 1 {
		return result, 1
	}
	panic("invalid input")
}
func new(str string) (result BigInt) {
	if len(str) == 0 || str == "-" || str == "-0" || str == "0" { // смотрю не пуст ли вход
		result.Body = []uint64{0}
		return
	}
	if str[0] == '-' { // проверяю знак
		result.Neg = true
		str = str[1:]
	}
	start, end := 0, 0
	var z1, z2, z3 BigInt
	z1.Body, z2.Body, z3.Body = []uint64{
		1,
		10,
		100,
		1000,
		10000,
		100000,
		1000000,
		10000000,
		100000000,
		1000000000,
		10000000000,
		100000000000,
		1000000000000,
		10000000000000,
		100000000000000,
		1000000000000000,
		10000000000000000,
		100000000000000000,
		1000000000000000000}, []uint64{0}, []uint64{0}
	for i := 0; ; i++ {
		end += word
		if end > len(str) {
			end = len(str)
		}
		segment := str[start:end]
		start = end
		val, _ := strconv.ParseUint(segment, 10, 64)
		z3.Body[0] = z1.Body[len(segment)]
		z2.Body[0] = val
		result.MulAbs(result, z3)
		result.AddAbs(result, z2)
		if end == len(str) {
			break
		}
	}
	return
}
func (result *BigInt) NewInt(str string) {
	strs, variant := parse(str)
	if variant == 1 {
		*result = new(str)
	} else if len(strs) == 2 {
		switch variant {
		case 2:
			var base BigInt
			*result = new(strs[0])
			base = new(strs[0])
			power, _ := strconv.ParseInt(strs[1], 10, 64)
			if power == 0 {
				result.Body = []uint64{1}
				return
			} else if power == 1 {
				return
			}
			for i := power - 1; i != 0; i -= 1 {
				result.MulAbs(*result, base)
			}
			return
		case 3:
			amount, _ := strconv.ParseInt(strs[1], 10, 64)
			add := strings.Repeat("0", int(amount))
			strs[0] = strs[0] + add
			*result = new(strs[0])
		}
	}
}
func (result *BigInt) Sub(value1, value2 BigInt) {
	if value2.Neg || value1.Neg { //this covers -- -+ +-
		switch {
		case value1.Neg && value2.Neg: //true,true ---=-+
			cmp := CmpAbs(value1, value2)
			switch {
			case cmp == 0: //skip calc cuz equal
				result.Body = []uint64{0}
				result.Neg = false
				return
			case cmp == 1: //val1 bigger
				result.SubAbs(value1, value2)
				result.Neg = true
				return
			case cmp == -1: //val1 smaller
				result.SubAbs(value2, value1)
				result.Neg = false
				return
			}
		case !value1.Neg && value2.Neg: //+--=++=+ sing +
			result.AddAbs(value1, value2)
			result.Neg = false
			return
		case value1.Neg && !value2.Neg: //--+=--=+ sing -
			result.AddAbs(value1, value2)
			result.Neg = true
			return

		}
	}
	// this to cover ++ case +-+=+-=-
	cmp := CmpAbs(value1, value2)
	switch cmp {
	case 0: //skip calc cuz equal
		result.Body = []uint64{0}
		return
	case 1: //val1 bigger
		result.SubAbs(value1, value2)
		result.Neg = false
		return
	case -1: //val1 smaller
		result.SubAbs(value2, value1)
		result.Neg = true
		return
	}
}
func (result *BigInt) Add(value1, value2 BigInt) {
	if value1.Neg || value2.Neg {
		cmp := CmpAbs(value1, value2)
		switch {
		case value1.Neg && value2.Neg: //both negative so -+-=+++ and sing switch
			result.AddAbs(value1, value2)
			result.Neg = true
			return
		case cmp == 0: // skip calc cuz equal
			result.Body = []uint64{0}
			return
		case cmp == -1: //val1 is smaller so
			result.SubAbs(value2, value1)
			return
		case cmp == 1: //val1 bigger so doing normalsub
			result.SubAbs(value1, value2)
			result.Neg = true
			return
		}
	}
	result.AddAbs(value1, value2)
	result.Neg = false
}
func (result *BigInt) Mul(value1, value2 BigInt) {}

func (result *BigInt) MulAbs(value1, value2 BigInt) {
	var hi, low, c1, c2 uint64
	result.Body = make([]uint64, max(len(value1.Body), len(value2.Body))*2)
	for z := 0; z < len(value2.Body); z++ {
		for i := 0; i < len(value1.Body); i++ {
			hi, low = bits.Mul64(value1.Body[i], value2.Body[z])
			result.Body[i+z], c1 = bits.Add64(result.Body[i+z], low, c1)
			result.Body[i+z+1], c2 = bits.Add64(result.Body[i+z+1], hi, c2)
		}
	}
	result.Body = trim(result.Body)
	result.Neg = false
}

func (result *BigInt) SubAbs(value1, value2 BigInt) {
	var carry uint64
	MinusCount := min(len(value1.Body), len(value2.Body))
	maxlen := max(len(value1.Body), len(value2.Body))
	result.Body = make([]uint64, maxlen)
	if len(value1.Body) == maxlen {
		copy(result.Body, value1.Body)
	} else {
		copy(result.Body, value2.Body)
	}
	for i := 0; i < MinusCount; i++ {
		result.Body[i], carry = bits.Sub64(value1.Body[i], value2.Body[i], carry)
		for z := 1; carry == 1; z++ {
			result.Body[i+z], carry = bits.Sub64(result.Body[i+z], carry, uint64(0))
		}
	}
	result.Body = trim(result.Body)
	result.Neg = false
}

func (result *BigInt) AddAbs(value1, value2 BigInt) {
	var carry uint64
	AddCount := min(len(value1.Body), len(value2.Body))
	maxlen := max(len(value1.Body), len(value2.Body))
	result.Body = make([]uint64, maxlen)
	if len(value1.Body) == maxlen {
		copy(result.Body, value1.Body)
	} else {
		copy(result.Body, value2.Body)
	}
	for i := 0; i < AddCount; i++ {
		result.Body[i], carry = bits.Add64(value1.Body[i], value2.Body[i], carry)
	}
	for carry == 1 {
		if max(len(value1.Body), len(value2.Body)) == AddCount {
			result.Body = append(result.Body, 1)
			break
		} else {
			result.Body[AddCount]++
			break
		}
	}
	result.Body = trim(result.Body)
	result.Neg = false
}
