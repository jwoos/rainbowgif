package main

import (
	"image"
	"image/color"
	"image/draw"
	"image/gif"
)

func staticTransform(img image.Image, format string, quantizer string, delay uint) (*gif.GIF, error) {
	transform := img.ColorModel() != color.RGBAModel

	bounds := img.Bounds()

	colors := make([]color.RGBA, (bounds.Max.Y - bounds.Min.Y) * (bounds.Max.X - bounds.Min.X))
	stride := bounds.Max.X - bounds.Min.X
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			c := img.At(x, y)
			if transform {
				colors[y * stride + x] = color.RGBAModel.Convert(c).(color.RGBA)
			} else {
				colors[y * stride + x] = c.(color.RGBA)
			}
		}
	}

	q := newQuantization(256)
	newColorsPtr, indexMap, err := q.quantize(quantizer, colors)
	if err != nil {
		return nil, err
	}

	pix := make([]uint8, len(colors))
	for i, paletteIndex := range indexMap {
		pix[i] = uint8(paletteIndex)
	}

	newColors := make([]color.Color, len(newColorsPtr))
	for i, c := range newColorsPtr {
		newColors[i] = *c
	}

	pi := image.NewPaletted(bounds, newColors)
	// dithering
	draw.FloydSteinberg.Draw(pi, bounds, img, image.Point{})
	pi.Stride = stride
	pi.Pix = pix

	gifImg := gif.GIF{
		Image: []*image.Paletted{pi},
		Delay: []int{int(delay)},
		LoopCount: 0,
	}

	return &gifImg, nil
}
