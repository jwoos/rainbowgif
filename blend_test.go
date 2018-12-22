package main


import (
	"testing"

	"github.com/lucasb-eyer/go-colorful"
)


func TestNewGradient(t *testing.T) {
	t.Run(
		"Zero color",
		func (innerT *testing.T) {
			gradient := newGradient(
				[]colorful.Color{},
			)

			if len(gradient.positions) != 0 {
				innerT.Errorf("Expected %v but got %v", 0, len(gradient.positions))
			}
		},
	)

	t.Run(
		"One color",
		func (innerT *testing.T) {
			gradient := newGradient(
				[]colorful.Color{
					{0, 0, 0},
				},
			)

			if len(gradient.positions) != 1 {
				innerT.Errorf("Expected %v but got %v", 1, len(gradient.positions))
			}

			if gradient.positions[0] != 0.0 {
				innerT.Errorf("Expected %v but got %v", 0.0, gradient.positions[0])
			}
		},
	)

	t.Run(
		"Two colors",
		func (innerT *testing.T) {
			gradient := newGradient(
				[]colorful.Color{
					{0, 0, 0},
					{1, 1, 1},
				},
			)

			if len(gradient.positions) != 2 {
				innerT.Errorf("Expected %v but got %v", 2, len(gradient.positions))
			}

			if gradient.positions[0] != 0.0 {
				innerT.Errorf("Expected %v but got %v", 0.0, gradient.positions[0])
			}

			if gradient.positions[1] != 1.0 {
				innerT.Errorf("%v", gradient.colors)
				innerT.Errorf("%v", gradient.positions)
				innerT.Errorf("Expected %v but got %v", 1.0, gradient.positions[1])
			}
		},
	)
}


func TestPositionSearch(t *testing.T) {
	t.Run(
		"One color",
		func (innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
			}
			gradient := newGradient(colors)

			for i := 0.0; i <= 1.0; i += 0.1 {
				returnedColors := gradient.positionSearch(i)

				if returnedColors[0] != colors[0] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[0],
						returnedColors[0],
					)
				}
			}
		},
	)

	t.Run(
		"Two colors",
		func (innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{1, 1, 1},
			}
			gradient := newGradient(colors)

			for i := 0.0; i <= 1.0; i += 0.1 {
				returnedColors := gradient.positionSearch(i)

				if returnedColors[0] != colors[0] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[0],
						returnedColors[0],
					)
				}

				if returnedColors[1] != colors[1] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[1],
						returnedColors[1],
					)
				}
			}
		},
	)

	t.Run(
		"Three colors",
		func (innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.5, 0.5, 0.5},
				{1, 1, 1},
			}
			gradient := newGradient(colors)

			for i := 0.0; i < 0.5; i += 0.1 {
				returnedColors := gradient.positionSearch(i)

				if returnedColors[0] != colors[0] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[0],
						returnedColors[0],
					)
				}

				if returnedColors[1] != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[1],
						returnedColors[1],
					)
				}
			}

			for i := 0.5; i <= 1.0; i += 0.1 {
				returnedColors := gradient.positionSearch(i)

				if returnedColors[0] != colors[1] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[1],
						returnedColors[0],
					)
				}

				if returnedColors[1] != colors[2] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[2],
						returnedColors[1],
					)
				}
			}
		},
	)

	t.Run(
		"Three colors",
		func (innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.33, 0.33, 0.33},
				{0.66, 0.66, 0.66},
				{1, 1, 1},
			}
			gradient := newGradient(colors)

			for i := 0.0; i <= 0.33; i += 0.03 {
				returnedColors := gradient.positionSearch(i)

				if returnedColors[0] != colors[0] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[0],
						returnedColors[0],
					)
				}

				if returnedColors[1] != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[1],
						returnedColors[1],
					)
				}
			}

			for i := 0.34; i < 0.66; i += 0.03 {
				returnedColors := gradient.positionSearch(i)

				if returnedColors[0] != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[1],
						returnedColors[0],
					)
				}

				if returnedColors[1] != colors[2] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[2],
						returnedColors[1],
					)
				}
			}

			for i := 0.67; i <= 1; i += 0.03 {
				returnedColors := gradient.positionSearch(i)

				if returnedColors[0] != colors[2] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[2],
						returnedColors[0],
					)
				}

				if returnedColors[1] != colors[3] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[3],
						returnedColors[0],
					)
				}
			}
		},
	)
}
