package main


import (
	"math"

	"github.com/lucasb-eyer/go-colorful"
)


type Gradient struct {
	colors []colorful.Color
	positions []float64
}


func newGradient(colors []colorful.Color) Gradient {
	gradient := Gradient{
		colors: make([]colorful.Color, len(colors)),
		positions: make([]float64, len(colors)),
	}
	copy(gradient.colors, colors)

	colorCount := len(gradient.colors) - 1

	if len(colors) == 1 {
		gradient.positions[0] = 0.0
	} else {
		// Distribute the colors evenly
		for i := 0; i <= colorCount; i++ {
			gradient.positions[i] = float64(i) / float64(colorCount)
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


func (gradient Gradient) positionSearch(position float64) []colorful.Color {
	length := len(gradient.colors) - 1
	base := 1.0 / float64(length)
	lowerIndex := int(math.Floor(position / base))

	sliced := gradient.colors[lowerIndex:]

	if len(sliced) > 2 {
		sliced = sliced[:2]
	}

	return sliced
}
