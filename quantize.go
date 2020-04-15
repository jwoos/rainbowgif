package main

import (
	"image/color"
	"sort"
	"github.com/dhconnelly/rtreego"
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

// for R-tree
type Point struct {
	location rtreego.Point
}

func (p *Point) Bounds() *rtreego.Rect {
	return p.location.ToRect(0.001)
}

func (q Quantization) populosity(colors []color.RGBA) []color.RGBA {
	if len(colors) < q.count {
		return colors
	}

	countMap := make(map[color.RGBA]int)

	for _, c := range colors {
		_, okay := countMap[c]
		if okay {
			countMap[c]++
		} else {
			countMap[c] = 1
		}
	}

	temp := make([]struct{
		color color.RGBA
		count int
	}, len(countMap))
	index := 0
	for k, v := range countMap {
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

	rt := rtreego.NewTree(3, 10, 30)

	for _, c := range temp {
		pt := Point{location: []float64{float64(c.color.R), float64(c.color.G), float64(c.color.B)}}
		rt.Insert(&pt)
	}

	palette := make([]color.RGBA, len(colors))
	for i, originalColors := range colors {
		pt := []float64{float64(originalColors.R), float64(originalColors.G), float64(originalColors.B)}
		nearestPt := rt.NearestNeighbor(pt).Bounds()
		palette[i] = color.RGBA{
			R: uint8(nearestPt.PointCoord(0)),
			G: uint8(nearestPt.PointCoord(1)),
			B: uint8(nearestPt.PointCoord(2)),
			A: 1,
		}
	}

	return palette
}

func (q Quantization) medianCut() {
	panic("Not implemented")
}

func (q Quantization) octree() {
	panic("Not implemented")
}

func (q Quantization) kmeans() {
	panic("Not implemented")
}
