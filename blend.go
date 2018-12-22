package main


import (
	"github.com/lucasb-eyer/go-colorful"
)


type Gradient struct {
	colors []colorful.Color
	positions []float32
}


func newGradient(colors []colorful.Color) Gradient {
	gradient := Gradient{
		colors: make([]colorful.Color, len(colors)),
		positions: make([]float32, len(colors)),
	}
	copy(gradient.colors, colors)

	colorCount := len(gradient.colors) - 1

	if len(colors) == 1 {
		gradient.positions[0] = 0.0
	} else {
		// Distribute the colors evenly
		for i := 0; i <= colorCount; i++ {
			gradient.positions[i] = float32(i) / float32(colorCount)
		}
	}

	return gradient
}


func (gradient Gradient) generate(frameCount int) []colorful.Color {
	generated := make([]colorful.Color, frameCount)

	/*
	 *for i := 0; int(i) < frameCount; i++ {
	 *    position := int(i) / frameCount
	 *}
	 */

	return generated
}


func (gradient Gradient) positionSearch(position float32) []int {
	var val float32 = 1.0
	var half float32 = val / 2.0
	var current []colorful.Color = gradient.colors
	var indices = []int{0}
	indices = append(indices, int(len(current) - 1))

	for len(current) > 0 {
		if len(current) <= 2 {
			return indices
		}

		if position == half {
			indices[1] = len(current) / 2

			if len(current) & 1 == 1 {
				// odd
				indices[0] = indices[1]
			} else {
				// even
				indices[0] = indices[1] - 1
			}

			return indices
		} else if position < half {
			indices[1] = len(current) / 2
		} else {
			indices[0] = len(current) / 2
		}

		current = current[indices[0]:indices[1]]
	}

	return indices
}
