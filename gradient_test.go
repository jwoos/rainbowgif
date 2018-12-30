package main

import (
	"testing"

	"github.com/lucasb-eyer/go-colorful"
)

func TestNewGradient(t *testing.T) {
	t.Run(
		"Zero color",
		func(innerT *testing.T) {
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
		func(innerT *testing.T) {
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
		func(innerT *testing.T) {
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
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
			}
			gradient := newGradient(colors)

			for i := 0.0; i <= 1.0; i += 0.1 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[0] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[0],
						returnedKeyFrames[0].color,
					)
				}
			}
		},
	)

	t.Run(
		"Two colors",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{1, 1, 1},
			}
			gradient := newGradient(colors)

			for i := 0.0; i <= 1.0; i += 0.1 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[0] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[0],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[1] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[1],
						returnedKeyFrames[1].color,
					)
				}
			}
		},
	)

	t.Run(
		"Three colors",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.5, 0.5, 0.5},
				{1, 1, 1},
			}
			gradient := newGradient(colors)

			for i := 0.0; i < 0.5; i += 0.1 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[0] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[0],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[1],
						returnedKeyFrames[1].color,
					)
				}
			}

			for i := 0.5; i <= 1.0; i += 0.1 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[1] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[1],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[2] {
					innerT.Errorf(
						"Expected %v but got %v",
						colors[2],
						returnedKeyFrames[1].color,
					)
				}
			}
		},
	)

	t.Run(
		"Three colors",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.33, 0.33, 0.33},
				{0.66, 0.66, 0.66},
				{1, 1, 1},
			}
			gradient := newGradient(colors)

			for i := 0.0; i <= 0.33; i += 0.03 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[0] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[0],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[1],
						returnedKeyFrames[1].color,
					)
				}
			}

			for i := 0.34; i < 0.66; i += 0.03 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[1],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[2] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[2],
						returnedKeyFrames[1].color,
					)
				}
			}

			for i := 0.67; i <= 1; i += 0.03 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[2] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[2],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[3] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[3],
						returnedKeyFrames[0].color,
					)
				}
			}
		},
	)

	t.Run(
		"Four colors",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.25, 0.25, 0.25},
				{0.5, 0.5, 0.5},
				{0.75, 0.75, 0.75},
				{1, 1, 1},
			}
			gradient := newGradient(colors)

			for i := 0.0; i < 0.25; i += 0.05 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[0] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[0],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[1],
						returnedKeyFrames[1].color,
					)
				}
			}

			for i := 0.26; i < 0.5; i += 0.05 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[1] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[1],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[2] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[2],
						returnedKeyFrames[1].color,
					)
				}
			}

			for i := 0.51; i < 0.75; i += 0.05 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[2] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[2],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[3] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[3],
						returnedKeyFrames[0].color,
					)
				}
			}

			for i := 0.76; i <= 1; i += 0.05 {
				returnedKeyFrames := gradient.positionSearch(i)

				if returnedKeyFrames[0].color != colors[3] {
					innerT.Errorf(
						"positionSearch(%f)[0] - expected %v but got %v",
						i,
						colors[3],
						returnedKeyFrames[0].color,
					)
				}

				if returnedKeyFrames[1].color != colors[4] {
					innerT.Errorf(
						"positionSearch(%f)[1] - expected %v but got %v",
						i,
						colors[4],
						returnedKeyFrames[0].color,
					)
				}
			}
		},
	)
}

func TestGenerate(t *testing.T) {
	t.Run(
		"Two colors - two frames",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{1, 1, 1},
			}
			gradient := newGradient(colors)
			generated := gradient.generate(2)

			if len(generated) != 2 {
				innerT.Errorf("Expected %v but got %v", 2, len(generated))
			}

			if generated[0] != colors[0] {
				innerT.Errorf("Expected %v but got %v", colors[0], generated[0])
			}

			if generated[1] != colors[1] {
				innerT.Errorf("Expected %v but got %v", colors[1], generated[1])
			}
		},
	)

	t.Run(
		"Two colors - three frames",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{1, 1, 1},
			}
			gradient := newGradient(colors)
			generated := gradient.generate(3)

			if len(generated) != 3 {
				innerT.Errorf("Expected %v but got %v", 2, len(generated))
			}

			if generated[0] != colors[0] {
				innerT.Errorf("Expected %v but got %v", colors[0], generated[0])
			}

			if generated[1] == colors[0] || generated[1] == colors[1] {
				innerT.Errorf("Expected %v but got %v", nil, generated[0])
			}

			if generated[2] != colors[1] {
				innerT.Errorf("Expected %v but got %v", colors[1], generated[2])
			}
		},
	)

	t.Run(
		"Two colors - four frames",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{1, 1, 1},
			}
			gradient := newGradient(colors)
			generated := gradient.generate(4)

			if len(generated) != 4 {
				innerT.Errorf("Expected %v but got %v", 2, len(generated))
			}

			if generated[0] != colors[0] {
				innerT.Errorf("Expected %v but got %v", colors[0], generated[0])
			}

			if generated[1] == colors[0] || generated[1] == colors[1] {
				innerT.Errorf("Expected %v but got %v", nil, generated[1])
			}

			if generated[2] == colors[0] || generated[2] == colors[1] {
				innerT.Errorf("Expected %v but got %v", nil, generated[2])
			}

			if generated[3] != colors[1] {
				innerT.Errorf("Expected %v but got %v", colors[1], generated[3])
			}
		},
	)

	t.Run(
		"Three colors - two frames",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.5, 0.5, 0.5},
				{1, 1, 1},
			}
			gradient := newGradient(colors)
			generated := gradient.generate(2)

			if len(generated) != 2 {
				innerT.Errorf("Expected %v but got %v", 2, len(generated))
			}

			if generated[0] != colors[0] {
				innerT.Errorf("Expected %v but got %v", colors[0], generated[0])
			}

			if generated[1] != colors[2] {
				innerT.Errorf("Expected %v but got %v", colors[2], generated[1])
			}
		},
	)

	t.Run(
		"Three colors - three frames",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.5, 0.5, 0.5},
				{1, 1, 1},
			}
			gradient := newGradient(colors)
			generated := gradient.generate(3)

			if len(generated) != 3 {
				innerT.Errorf("Expected %v but got %v", 3, len(generated))
			}

			if generated[0] != colors[0] {
				innerT.Errorf("Expected %v but got %v", colors[0], generated[0])
			}

			/*
			 *if generated[1] != colors[1] {
			 *    innerT.Errorf("Expected %v but got %v", colors[1], generated[1])
			 *}
			 */

			if generated[2] != colors[2] {
				innerT.Errorf("Expected %v but got %v", colors[2], generated[2])
			}
		},
	)

	t.Run(
		"Three colors - four frames",
		func(innerT *testing.T) {
			colors := []colorful.Color{
				{0, 0, 0},
				{0.5, 0.5, 0.5},
				{1, 1, 1},
			}
			gradient := newGradient(colors)
			generated := gradient.generate(4)

			if len(generated) != 4 {
				innerT.Errorf("Expected %v but got %v", 3, len(generated))
			}

			/*
			 *if generated[0] != colors[0] {
			 *    innerT.Errorf("Expected %v but got %v", colors[0], generated[0])
			 *}
			 */

			if generated[0] != colors[0] {
				innerT.Errorf("Expected %v but got %v", colors[0], generated[0])
			}

			if generated[3] != colors[2] {
				innerT.Errorf("Expected %v but got %v", colors[2], generated[3])
			}
		},
	)
}
