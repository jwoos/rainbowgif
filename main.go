package main


import (
	"fmt"
	"image/color"
	"image/gif"
	"os"

	"github.com/lucasb-eyer/go-colorful"
)


func main() {
	file, err := os.Open("test.gif")
	if err != nil {
		fmt.Println("Error opening file: ", err)
		os.Exit(1)
	}

	image, err := gif.DecodeAll(file)
	if err != nil {
		fmt.Println("Error decoding: ", err)
		os.Exit(1)
	}

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
			r, g, b, alpha := pixel.RGBA()

			if alpha == 0 {
				newPalette[pixelIndex] = pixel
				continue
			}

			convertedPixel := colorful.Color{
				float64(r) / 255,
				float64(g) / 255,
				float64(b) / 255,
			}

			blendedPixel,_ := blendAlpha(overlayColors[frameIndex], 0.5, convertedPixel, 1)
			blendedR, blendedG, blendedB, _ := blendedPixel.RGBA()
			newPalette[pixelIndex] = color.NRGBA{
				uint8(blendedR),
				uint8(blendedG),
				uint8(blendedB),
				255,
			}
		}

		frame.Palette = newPalette
	}

	file, err = os.OpenFile("test-output.gif", os.O_RDWR | os.O_CREATE, 0644)
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
}
