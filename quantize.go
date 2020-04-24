package main

import (
	"errors"
	"image/color"
	"math"
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

	palette := make(map[color.RGBA]struct {
		index int
		count int
	})

	for _, c := range colors {
		v, okay := palette[c]
		if okay {
			v.count++
			palette[c] = v
		} else {
			palette[c] = struct {
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

func (q Quantizer) medianCut(colors []color.RGBA) ([]*color.RGBA, []int) {
	if len(colors) < q.count {
		return q.identity(colors)
	}

	// for use later on
	indices := make(map[color.RGBA][]int)
	uniqueColors := make([]*color.RGBA, 0)

	for i, c := range colors {
		_, okay := indices[c]
		if okay {
			indices[c] = append(indices[c], i)
		} else {
			indices[c] = []int{i}
			uniqueColors = append(uniqueColors, &color.RGBA{R: c.R, G: c.G, B: c.B, A: c.A})
		}
	}

	depth := int(math.Round(math.Log2(float64(q.count))))
	actualCount := int(math.Pow(2, float64(depth)))
	palette := make([]*color.RGBA, actualCount)
	indexMapping := make([]int, len(colors))

	uniqueColors, mappedColors := q.medianCutSplit(uniqueColors, depth)

	temp := make(map[*color.RGBA]int)
	index := 0
	for _, c := range mappedColors {
		_, okay := temp[c]
		if !okay {
			palette[index] = c
			temp[c] = index
			index++
		}
	}

	for i := range uniqueColors {
		colorPtr := uniqueColors[i]
		mappedColor := mappedColors[i]
		originalIndices := indices[*colorPtr]


		for _, x := range originalIndices {
			indexMapping[x] = temp[mappedColor]
		}
	}


	return palette, indexMapping
}

func (q Quantizer) medianCutSplit(bucket []*color.RGBA, depth int) ([]*color.RGBA, []*color.RGBA) {
	if len(bucket) == 0 {
		return nil, nil
	}

	if depth == 0 {
		// average all the pixels in the bucket and return
		sum := []uint{0, 0, 0, 1}
		for _, c := range bucket {
			sum[0] += uint(c.R)
			sum[1] += uint(c.G)
			sum[2] += uint(c.B)
			sum[3] *= uint(c.A)
		}

		if sum[3] != 0 {
			sum[3] = 255
		}

		avg := &color.RGBA{
			R: uint8(sum[0] / uint(len(bucket))),
			G: uint8(sum[1] / uint(len(bucket))),
			B: uint8(sum[2] / uint(len(bucket))),
			A: uint8(sum[3]),
		}

		palette := make([]*color.RGBA, len(bucket))
		for i := range palette {
			palette[i] = avg
		}

		return bucket, palette
	}

	r := []int{-1, 0}
	g := []int{-1, 0}
	b := []int{-1, 0}

	for _, c := range bucket {
		cR := int(c.R)
		if r[0] == -1 || cR < r[0] {
			r[0] = cR
		}
		if cR > r[1] {
			r[1] = cR
		}

		cG := int(c.G)
		if g[0] == -1 || cG < g[0] {
			g[0] = cG
		}
		if cG > g[1] {
			g[1] = cG
		}

		cB := int(c.B)
		if b[0] == -1 || cB < b[0] {
			b[0] = cB
		}
		if cB > b[1] {
			b[1] = cB
		}
	}

	rDiff := r[1] - r[0]
	gDiff := g[1] - g[0]
	bDiff := b[1] - b[0]

	if rDiff > gDiff && rDiff > bDiff {
		// red
		sort.Slice(bucket, func(i int, j int) bool {
			return bucket[i].R < bucket[j].R
		})
	} else if gDiff > bDiff {
		// green
		sort.Slice(bucket, func(i int, j int) bool {
			return bucket[i].G < bucket[j].G
		})
	} else {
		// blue
		sort.Slice(bucket, func(i int, j int) bool {
			return bucket[i].B < bucket[j].B
		})
	}

	aPalette, aMappedcolor := q.medianCutSplit(bucket[:len(bucket) / 2 + 1], depth - 1)
	bPalette, bMappedColor := q.medianCutSplit(bucket[len(bucket) / 2 + 1:], depth - 1)

	palette := append(aPalette, bPalette...)
	mappedColor := append(aMappedcolor, bMappedColor...)

	return palette, mappedColor
}


func (q Quantizer) octree([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}

func (q Quantizer) kmeans([]color.RGBA) ([]*color.RGBA, []int) {
	panic("Not implemented")
}
