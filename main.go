package main


import (
	"flag"
	"fmt"
	"image/color"
	"image/gif"
	"os"

	"github.com/lucasb-eyer/go-colorful"
)


func main() {
	input := flag.String("input", "", "The input file name")
	output := flag.String("output", "", "The output file name")
	flag.Parse()

	file, err := os.Open(*input)
	if err != nil {
		fmt.Println("Error opening file: ", err)
		os.Exit(1)
	}

	image, err := gif.DecodeAll(file)
	if err != nil {
		fmt.Println("Error decoding: ", err)
		os.Exit(1)
	}
	file.Close()

	frameCount := len(image.Image)

	// ROYGBV
	gradient := newGradient(
		[]colorful.Color{
			{1, 0, 0},
			{1, 127.0 / 255.0, 0},
			{1, 1, 0},
			{0, 1, 0},
			{0, 0, 1},
			{139.0 / 255.0, 0, 1},
			{0.9, 0, 0},
		},
	)
	overlayColors := gradient.generate(frameCount)

	/* frame is just `*image.Paletted`
	 * `color.Palette` is just `[]color.Color`
	 * `color.Color` is an interface implementing `RGBA()`
	 */
	for frameIndex, frame := range image.Image {
		originalPalette := frame.Palette
		newPalette := make([]color.Color, len(frame.Palette))

		for pixelIndex, pixel := range originalPalette {
			_, _, _, alpha := pixel.RGBA()
			convertedPixel, ok := colorful.MakeColor(pixel)

			if alpha == 0 || !ok {
				newPalette[pixelIndex] = pixel
				continue
			}

			convertedPixel = convertedPixel.Clamped()

			blendedPixel := blendColor(overlayColors[frameIndex], convertedPixel)

			blendedR, blendedG, blendedB := blendedPixel.RGB255()
			newPalette[pixelIndex] = color.NRGBA{
				blendedR,
				blendedG,
				blendedB,
				255,
			}
		}

		frame.Palette = newPalette
	}

	file, err = os.OpenFile(*output, os.O_RDWR | os.O_CREATE, 0644)
	if err != nil {
		fmt.Println("Error opening file: ", err)
		os.Exit(1)
	}

	image.Config.ColorModel = nil
	image.BackgroundIndex = 0

	err = gif.EncodeAll(file, image)
	if err != nil {
		fmt.Println("Error encoding image: ", err)
		os.Exit(1)
	}
	file.Close()
}
