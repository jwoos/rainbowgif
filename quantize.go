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

// TODO this should return a mapping of old colors to new colors
func (q Quantization) scalar(colors []color.RGBA) []color.RGBA {
	if len(colors) < q.count {
		return colors
	}

	palette := make(map[color.RGBA]int)
	for _, c := range colors {
		// pack R into 3 bits, G into 3 bits, and B into 2 bits
		newColor := color.RGBA{
			R: c.R & (0b11100000),
			G: c.G & (0b00011100),
			B: c.B & (0b00000011),
			A: 1,
		}

		palette[newColor] = 0
	}

	paletteSlice := make([]color.RGBA, len(palette))
	index := 0
	for k, _ := range palette {
		paletteSlice[index] = k
		index++
	}

	return paletteSlice
}

func (q Quantization) populosity(colors []color.RGBA) []color.RGBA {
	if len(colors) < q.count {
		return colors
	}

	palette := make(map[color.RGBA]int)

	for _, c := range colors {
		_, okay := palette[c]
		if okay {
			palette[c]++
		} else {
			palette[c] = 1
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
		return temp[i].count > temp[j].count
	})

	// lose any extra colors
	if len(temp) > q.count {
		temp = temp[:q.count]
	}


	// TODO do a 3D search to return a palette mapping
	return colors
}

func (q Quantization) medianCut() {

}

func (q Quantization) octree() {

}

func (q Quantization) kmeans() {

}
