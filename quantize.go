package main

import (
	"image/color"
	"sort"
)

type Quantization struct {
	count int
}

func newQuantization(count int) Quantization {
	return Quantization{
		count: count,
	}
}

func (q Quantization) scalar(colors []color.RGBA) []color.RGBA {
	palette := make(map[color.RGBA]int)
	for _, c := range colors {
		// pack R into 3 bits, G into 3 bits, and B into 2 bits
		newColor := color.RGBA{
			R: c.R & (0b11100000),
			G: c.G & (0b00011100),
			B: c.B & (0b00000011),
			A: 1,
		}

		_, okay := palette[newColor]
		if !okay {
			palette[newColor] = 1
		} else {
			palette[newColor]++
		}
	}

	temp := make([]struct{
		color color.RGBA
		count int
	}, len(palette))
	index := 0
	for k, v := range palette {
		temp[index] = struct{
			color color.RGBA
			count int
		}{color: k, count: v}
		index++
	}

	sort.Slice(temp, func(i int, j int) bool {
		return temp[i].count < temp[j].count
	})

	paletteSlice := make([]color.RGBA, len(temp))
	for i, x := range temp {
		paletteSlice[i] = x.color
	}

	return paletteSlice
}

func (q Quantization) population(colors []color.RGBA) {
}

func (q Quantization) medianCut() {

}

func (q Quantization) octree() {

}

func (q Quantization) kmeans() {

}
