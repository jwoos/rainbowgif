package main


import (
	"github.com/lucasb-eyer/go-colorful"
)

func blendAlpha(top colorful.Color, topAlpha float64, bottom colorful.Color, bottomAlpha float64) (colorful.Color, float64) {
	alphaDelta := (1 - topAlpha) * bottomAlpha

	alpha := alphaDelta + topAlpha
	red := (alphaDelta * bottom.R + topAlpha * top.R) / alpha
	green := (alphaDelta * bottom.G + topAlpha * top.G) / alpha
	blue := (alphaDelta * bottom.B + topAlpha * top.B) / alpha

	return colorful.Color{R: red, G: green, B: blue}, alpha
}
