package main

import (
	"errors"
	"flag"
	"fmt"
	"image"
	"image/color"
	"image/gif"
	"os"
	"runtime"
	"strings"

	"github.com/lucasb-eyer/go-colorful"
)

/* frame is just `*image.Paletted`
 * `color.Palette` is just `[]color.Color`
 * `color.Color` is an interface implementing `RGBA()`
 */
func prepareFrame(index int, frame *image.Paletted, overlayColor colorful.Color) {
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

		blendedPixel := blendColor(overlayColor, convertedPixel)

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

func parseGradientColors(gradientColors string) ([]colorful.Color, error) {
	var colors []colorful.Color

	if len(gradientColors) != 0 {
		colorHexes := strings.Split(gradientColors, ",")
		colors = make([]colorful.Color, len(colorHexes))
		for i, hex := range colorHexes {
			color, err := colorful.Hex("#" + hex)
			if err != nil {
				return nil, errors.New(fmt.Sprintf("Invalid color: %s", hex))
			}
			colors[i] = color
		}
	} else {
		// ROYGBV
		colors = []colorful.Color{
			{1, 0, 0},
			{1, 127.0 / 255.0, 0},
			{1, 1, 0},
			{0, 1, 0},
			{0, 0, 1},
			{139.0 / 255.0, 0, 1},
			{0.9, 0, 0},
		}
	}

	return colors, nil
}

func main() {
	var threads int
	flag.IntVar(&threads, "threads", runtime.NumCPU(), "The number of go threads to use")

	var gradientColors string
	flag.StringVar(&gradientColors, "gradient", "", "A list of colors in hex without # separated by comma to use as the gradient")

	flag.Parse()

	if threads <= 0 {
		fmt.Println("Thread count must be at least 1")
		os.Exit(1)
	}

	colors, err := parseGradientColors(gradientColors)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	positionalArgs := flag.Args()

	if len(positionalArgs) != 2 {
		fmt.Println("Expected two positional arguments: input and output")
		os.Exit(1)
	}

	input := positionalArgs[0]
	output := positionalArgs[1]

	file, err := os.Open(input)
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

	gradient := newGradient(colors)
	overlayColors := gradient.generate(frameCount)

	framesPerThread := len(image.Image)/threads + 1
	ch := make(chan int)
	barrier := 0

	for i := 0; i < threads; i++ {
		go func(base int) {
			for j := 0; j < framesPerThread; j++ {
				index := base*framesPerThread + j

				if index >= len(image.Image) {
					break
				}

				// do actual work in here
				prepareFrame(index, image.Image[index], overlayColors[index])
			}

			// thread is done
			ch <- 1
		}(i)
	}

	// wait for all threads to synchronize
	for barrier != threads {
		barrier += <-ch
	}

	file, err = os.OpenFile(output, os.O_RDWR|os.O_CREATE, 0644)
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
