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
				innerT.Errorf("Expected: %v and got %v", 0, len(gradient.positions))
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
				innerT.Errorf("Expected: %v and got %v", 1, len(gradient.positions))
			}

			if gradient.positions[0] != 0.0 {
				innerT.Errorf("Expected: %v and got %v", 0.0, gradient.positions[0])
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
				innerT.Errorf("Expected: %v and got %v", 2, len(gradient.positions))
			}

			if gradient.positions[0] != 0.0 {
				innerT.Errorf("Expected: %v and got %v", 0.0, gradient.positions[0])
			}

			if gradient.positions[1] != 1.0 {
				innerT.Errorf("%v", gradient.colors)
				innerT.Errorf("%v", gradient.positions)
				innerT.Errorf("Expected: %v and got %v", 1.0, gradient.positions[1])
			}
		},
	)
}
