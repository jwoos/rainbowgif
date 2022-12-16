package main

import (
	"fmt"
	"math"

	"github.com/lucasb-eyer/go-colorful"
)

type Gradient struct {
	colors    []colorful.Color
	positions []float64
}

type GradientKeyFrame struct {
	color    colorful.Color
	position float64
	index    int
}

func newGradient(colors []colorful.Color, wrap bool) Gradient {
	var gradient Gradient

	if wrap && len(colors) > 1 {
		// wrap around
		gradient = Gradient{
			colors:    make([]colorful.Color, len(colors)+1),
			positions: make([]float64, len(colors)+1),
		}
		copy(gradient.colors, colors)
		gradient.colors[len(colors)] = colors[0]
	} else {
		gradient = Gradient{
			colors:    make([]colorful.Color, len(colors)),
			positions: make([]float64, len(colors)),
		}
		copy(gradient.colors, colors)
	}

	colorCount := len(gradient.colors) - 1

	if len(gradient.colors) == 1 {
		gradient.positions[0] = 0.0
	} else {
		// Distribute the colors evenly
		for i := 0; i <= colorCount; i++ {
			gradient.positions[i] = float64(i) / float64(colorCount)
		}
	}

	return gradient
}

func (gradient Gradient) generate(frameCount uint) []colorful.Color {
	generated := make([]colorful.Color, frameCount)

	for i := uint(0); i < frameCount; i++ {
		position := float64(i) / float64(frameCount)
		keyframes := gradient.positionSearch(position)

		if len(keyframes) == 1 {
			generated[i] = keyframes[0].color.Clamped()
		} else {
			relativePosition := (position - keyframes[0].position) / (keyframes[1].position - keyframes[0].position)
			generated[i] = keyframes[0].color.BlendHcl(keyframes[1].color, relativePosition).Clamped()
		}
	}

	return generated
}

func (gradient Gradient) positionSearch(position float64) []GradientKeyFrame {
	length := len(gradient.colors) - 1
	base := 1.0 / float64(length)
	lowerIndex := int(math.Floor(position / base))

	sliced := gradient.colors[lowerIndex:]

	fmt.Println(length, base, lowerIndex, position, gradient.positions[lowerIndex]);

	if len(sliced) >= 2 {
		sliced = sliced[:2]

		return []GradientKeyFrame{
			{
				color:    sliced[0],
				position: gradient.positions[lowerIndex],
				index:    lowerIndex,
			},
			{
				color:    sliced[1],
				position: gradient.positions[lowerIndex+1],
				index:    lowerIndex + 1,
			},
		}
	}

	return []GradientKeyFrame{
		{
			color:    sliced[0],
			position: gradient.positions[lowerIndex],
			index:    lowerIndex,
		},
	}
}
