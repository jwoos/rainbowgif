package main

import (
	"errors"
	"image/color"
	"sort"
)

type Quantizer struct {
	count int
}

func newQuantizer(count int) Quantizer {
	return Quantizer{
		count: count,
	}
}

func (q Quantizer) quantize(algo string, colors []color.RGBA) ([]*color.RGBA, []int, error) {
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
		return nil, nil, errors.New("Invalid quantizer")
	}

	return palette, mapping, nil
}

/* helper method to just transform input to the appropriate output.
 * Generates a proper palette from any given input.
 */
func (q Quantizer) identity(colors []color.RGBA) ([]*color.RGBA, []int) {
	palette := make(map[color.RGBA]struct {
		addr  *color.RGBA
		index int
	})
	paletteSlice := make([]*color.RGBA, 0)
	indexMapping := make([]int, len(colors))

	for i, c := range colors {
		colorInfo, okay := palette[c]
		if !okay {
			colorInfo = struct {
				addr  *color.RGBA
				index int
			}{
				addr:  &c,
				index: len(paletteSlice),
			}
			palette[c] = colorInfo
			paletteSlice = append(paletteSlice, colorInfo.addr)
		}

		indexMapping[i] = colorInfo.index
	}

	return paletteSlice, indexMapping
}

func (q Quantizer) scalar(colors []color.RGBA) ([]*color.RGBA, []int) {
	if len(colors) < q.count {
		return q.identity(colors)
	}

	palette := make(map[color.RGBA]struct {
		addr  *color.RGBA
		index int
	})
	paletteSlice := make([]*color.RGBA, 0)
	indexMapping := make([]int, len(colors))
	for i, c := range colors {
		// pack R into 3 bits, G into 3 bits, and B into 2 bits
		newColor := &color.RGBA{
			R: c.R & (0b11100000),
			G: c.G & (0b11100000),
			B: c.B & (0b11000000),
			A: c.A,
		}

		colorInfo, okay := palette[*newColor]
		if !okay {
			colorInfo = struct {
				addr  *color.RGBA
				index int
			}{
				addr:  newColor,
				index: len(paletteSlice),
			}
			palette[*newColor] = colorInfo
			paletteSlice = append(paletteSlice, colorInfo.addr)
		}

		indexMapping[i] = colorInfo.index
	}

	return paletteSlice, indexMapping
}

func (q Quantizer) populosity(colors []color.RGBA) ([]*color.RGBA, []int) {
	if len(colors) < q.count {
		return q.identity(colors)
	}

	palette := make(map[color.RGBA]struct{
		index int
		count int
	})

	for _, c := range colors {
		v, okay := palette[c]
		if okay {
			v.count++
			palette[c] = v
		} else {
			palette[c] = struct{
				index int
				count int
			}{
				index: -1,
				count: 1,
			}
		}
	}

	// no need to do any extra work, we have all the colors we need
	if len(palette) <= q.count {
		return q.identity(colors)
	}

	sorted := make([]struct {
		color color.RGBA
		count int
	}, len(palette))
	index := 0
	for k, v := range palette {
		sorted[index] = struct {
			color color.RGBA
			count int
		}{color: k, count: v.count}
		index++
	}

	sort.Slice(sorted, func(i int, j int) bool {
		return sorted[i].count > sorted[j].count
	})

	// lose any extra colors
	if len(sorted) > q.count {
		sorted = sorted[:q.count]
	}

	tempPalette := make(color.Palette, len(sorted))
	for i, c := range sorted {
		tempPalette[i] = c.color
	}

	paletteSlice := make([]*color.RGBA, 0)
	indexMapping := make([]int, len(colors))

	// TODO toss this into a goroutine - `Convert` is essentially a linear search
	for i, originalColor := range colors {
		converted := tempPalette.Convert(originalColor).(color.RGBA)

		colorInfo := palette[converted]
		if colorInfo.index == -1 {
			colorInfo.index = len(paletteSlice)
			paletteSlice = append(paletteSlice, &color.RGBA{R: converted.R, G: converted.G, B: converted.B, A: originalColor.A})
			palette[converted] = colorInfo
		}

		indexMapping[i] = colorInfo.index
	}

	return paletteSlice, indexMapping
}

func (q Quantizer) medianCut([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}

func (q Quantizer) octree([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}

func (q Quantizer) kmeans([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}
