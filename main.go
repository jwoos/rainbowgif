package main

import (
	"flag"
	"fmt"
	"image"
	"image/color"
	"image/gif"
	"os"
	"runtime"

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

func main() {
	var input string
	flag.StringVar(&input, "input", "", "The input file name")

	var output string
	flag.StringVar(&output, "output", "", "The output file name")

	var threads int
	flag.IntVar(&threads, "threads", runtime.NumCPU(), "The number of go threads to use")

	flag.Parse()

	if threads <= 0 {
		fmt.Println("Thread count must be at least 1")
		os.Exit(1)
	}

	if input == "" || output == "" {
		fmt.Println("Input and output must be specified")
		os.Exit(1)
	}

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
