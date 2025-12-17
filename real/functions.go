package real

func Cmp(value1, value2 BigInt) int8 {
	switch {
	case value1.Neg && value2.Neg:
		return CmpAbs(value2, value1)
	case value1.Neg && !value2.Neg:
		return -1
	case !value1.Neg && value2.Neg:
		return 1
	case !value1.Neg && !value2.Neg:
		return CmpAbs(value1, value2)
	}
	panic("escape from Cmp")
}
func CmpAbs(value1, value2 BigInt) int8 {
	l1, l2 := len(value1.Body), len(value2.Body)
	switch {
	case l1 > l2:
		return 1
	case l1 < l2:
		return -1
	case l1 == l2:
		l1 -= 1
		for ; l1 >= 0; l1-- {
			switch {
			case value1.Body[l1] > value2.Body[l1]:
				return 1
			case value1.Body[l1] < value2.Body[l1]:
				return -1
			}

		}
	}
	return 0
}
func trim(value []uint64) []uint64 {
	if len(value) == 0 {
		panic("0 lenght value")
	}
	i := len(value) - 1
	for value[i] == 0 && i > 0 {
		value = value[:i]
		i--
		if i == 0 {
			break
		}
	}
	return value
}
func (result *BigInt) Pow(value1, value2 BigInt) {}
