package main


import (
	"testing"

	"github.com/lucasb-eyer/go-colorful"
)

func TestBlendNormal(t *testing.T) {
	t.Run(
		"Top alpha 1 - bottom alpha 1",
		func (innerT *testing.T) {
			topColor := colorful.Color{0, 0, 0}
			topAlpha := 1.0
			bottomColor := colorful.Color{1, 1, 1}
			bottomAlpha := 1.0

			color, alpha := blendNormal(
				topColor,
				topAlpha,
				bottomColor,
				bottomAlpha,
			)

			if color != topColor {
				innerT.Errorf("Expected %v but got %v", topColor, color)
			}

			if alpha != 1 {
				innerT.Errorf("Expected %v but got %v", topAlpha, alpha)
			}
		},
	)

	t.Run(
		"Top alpha 0 - bottom alpha 1",
		func (innerT *testing.T) {
			topColor := colorful.Color{0, 0, 0}
			topAlpha := 0.0
			bottomColor := colorful.Color{1, 1, 1}
			bottomAlpha := 1.0

			color, alpha := blendNormal(
				topColor,
				topAlpha,
				bottomColor,
				bottomAlpha,
			)

			if color != bottomColor {
				innerT.Errorf("Expected %v but got %v", bottomColor, color)
			}

			if alpha != bottomAlpha {
				innerT.Errorf("Expected %v but got %v", bottomAlpha, alpha)
			}
		},
	)

	t.Run(
		"Top alpha 0.5 - bottom alpha 0.5",
		func (innerT *testing.T) {
			topColor := colorful.Color{0, 0, 0}
			topAlpha := 0.5
			bottomColor := colorful.Color{1, 1, 1}
			bottomAlpha := 0.5

			color, alpha := blendNormal(
				topColor,
				topAlpha,
				bottomColor,
				bottomAlpha,
			)

			if color == bottomColor || color == topColor {
				innerT.Errorf("Expected %v but got %v", nil, color)
			}

			if alpha != 0.75 {
				innerT.Errorf("Expected %v but got %v", 1, alpha)
			}
		},
	)

	t.Run(
		"Top alpha 0.5 - bottom alpha 1.0",
		func (innerT *testing.T) {
			topColor := colorful.Color{0, 0, 0}
			topAlpha := 0.5
			bottomColor := colorful.Color{1, 1, 1}
			bottomAlpha := 1.0

			color, alpha := blendNormal(
				topColor,
				topAlpha,
				bottomColor,
				bottomAlpha,
			)

			if color == bottomColor || color == topColor {
				innerT.Errorf("Expected %v but got %v", nil, color)
			}

			if alpha != 1.0 {
				innerT.Errorf("Expected %v but got %v", 1, alpha)
			}
		},
	)
}
