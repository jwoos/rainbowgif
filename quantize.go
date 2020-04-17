package main

import (
	"errors"
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

func (q Quantization) quantize(algo string, colors []color.RGBA) ([]*color.RGBA, []int, error) {
	var palette []*color.RGBA
	var mapping []int

	switch algo {
	case "scalar":
		palette, mapping = q.scalar(colors)
	case "populosity":
		palette, mapping = q.populosity(colors)
	case "mediancut":
		palette, mapping = q.medianCut(colors)
	case "octree":
		palette, mapping = q.octree(colors)
	case "kmeans":
		palette, mapping = q.kmeans(colors)
	default:
		return nil, nil, errors.New("Invalid quantization algorithm")
	}

	return palette, mapping, nil
}

/* helper method to just transform input to the appropriate output.
 * Generates a proper palette from any given input.
 */
func (q Quantization) identity(colors []color.RGBA) ([]*color.RGBA, []int) {
	palette := make(map[color.RGBA]struct{
		addr *color.RGBA
		index int
	})
	paletteSlice := make([]*color.RGBA, len(colors))
	indexMapping := make([]int, len(colors))

	for i, c := range colors {
		colorInfo, okay := palette[c]
		if !okay {
			colorInfo = struct{
				addr *color.RGBA
				index int
			}{
				addr: &c,
				index: len(paletteSlice),
			}
			palette[c] = colorInfo
			paletteSlice = append(paletteSlice, colorInfo.addr)
		}

		indexMapping[i] = colorInfo.index
	}

	return paletteSlice, indexMapping
}

func (q Quantization) scalar(colors []color.RGBA) ([]*color.RGBA, []int) {
	if len(colors) < q.count {
		return q.identity(colors)
	}

	palette := make(map[color.RGBA]struct{
		addr *color.RGBA
		index int
	})
	paletteSlice := make([]*color.RGBA, 0)
	indexMapping := make([]int, len(colors))
	for i, c := range colors {
		// pack R into 3 bits, G into 3 bits, and B into 2 bits
		newColor := color.RGBA{
			R: c.R & (0b11100000),
			G: c.G & (0b00011100),
			B: c.B & (0b00000011),
			A: 1,
		}

		colorInfo, okay := palette[newColor]
		if !okay {
			colorInfo = struct{
				addr *color.RGBA
				index int
			}{
				addr: &newColor,
				index: len(paletteSlice),
			}
			palette[newColor] = colorInfo
			paletteSlice = append(paletteSlice, colorInfo.addr)
		}

		indexMapping[i] = colorInfo.index
	}

	return paletteSlice, indexMapping
}

// for R-tree
type Point struct {
	location rtreego.Point
}

func (p *Point) Bounds() *rtreego.Rect {
	return p.location.ToRect(0.001)
}

func (q Quantization) populosity(colors []color.RGBA) ([]*color.RGBA, []int) {
	if len(colors) < q.count {
		return q.identity(colors)
	}

	palette := make(map[color.RGBA]struct{
		addr *color.RGBA
		index int
		count int
	})

	for _, c := range colors {
		v, okay := palette[c]
		if okay {
			v.count++
		} else {
			palette[c] = struct{
				addr *color.RGBA
				index int
				count int
			}{
				addr: &c,
				index: -1,
				count: 1,
			}
		}
	}

	// no need to do any extra work, we have all the colors we need
	if len(palette) >= q.count {
		return q.identity(colors)
	}

	sorted := make([]struct{
		color *color.RGBA
		count int
	}, len(palette))
	index := 0
	for k, v := range palette {
		sorted[index] = struct{
			color *color.RGBA
			count int
		}{color: &k, count: v.count}
		index++
	}

	sort.Slice(sorted, func(i int, j int) bool {
		return sorted[i].count > sorted[j].count
	})

	// lose any extra colors
	if len(sorted) > q.count {
		sorted = sorted[:q.count]
	}

	rt := rtreego.NewTree(3, 10, 30)

	for _, c := range sorted {
		pt := Point{location: []float64{float64(c.color.R), float64(c.color.G), float64(c.color.B)}}
		rt.Insert(&pt)
	}

	paletteSlice := make([]*color.RGBA, 0)
	indexMapping := make([]int, len(colors))

	for i, originalColor := range colors {
		pt := []float64{float64(originalColor.R), float64(originalColor.G), float64(originalColor.B)}
		nearestPt := rt.NearestNeighbor(pt).Bounds()
		tempColor := color.RGBA{
			R: uint8(nearestPt.PointCoord(0)),
			G: uint8(nearestPt.PointCoord(1)),
			B: uint8(nearestPt.PointCoord(2)),
			A: 1,
		}

		colorInfo := palette[tempColor]
		if colorInfo.index == -1 {
			colorInfo.index = len(paletteSlice)
			paletteSlice = append(paletteSlice, colorInfo.addr)
		}

		indexMapping[i] = colorInfo.index
	}

	return paletteSlice, indexMapping
}

func (q Quantization) medianCut([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}

func (q Quantization) octree([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}

func (q Quantization) kmeans([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}
