package main

import (
	"errors"
	"flag"
	"fmt"
	"image"
	"image/color"
	"image/gif"
	"image/jpeg"
	"image/png"
	"os"
	"runtime"
	"strings"

	"github.com/lucasb-eyer/go-colorful"
)

func prepareFrame(src *image.Paletted, dst *image.Paletted, overlayColor colorful.Color) {
	dst.Pix = src.Pix
	dst.Stride = src.Stride

	for pixelIndex, pixel := range src.Palette {
		_, _, _, alpha := pixel.RGBA()
		convertedPixel, ok := colorful.MakeColor(pixel)

		if alpha == 0 || !ok {
			dst.Palette[pixelIndex] = pixel
			continue
		}

		convertedPixel = convertedPixel.Clamped()

		blendedPixel := blendColor(overlayColor, convertedPixel)

		blendedR, blendedG, blendedB := blendedPixel.RGB255()
		dst.Palette[pixelIndex] = color.NRGBA{
			blendedR,
			blendedG,
			blendedB,
			255,
		}
	}
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
		}
	}

	return colors, nil
}

func main() {
	// register image formats
	image.RegisterFormat("jpg", "\xFF\xD8", jpeg.Decode, jpeg.DecodeConfig)
	image.RegisterFormat("png", "\x89\x50\x4E\x47\x0D\x0A\x1A\x0A", png.Decode, png.DecodeConfig)

	var threads uint
	flag.UintVar(&threads, "threads", uint(runtime.NumCPU())/2, "The number of go threads to use")

	var gradientColors string
	flag.StringVar(&gradientColors, "gradient", "", "A list of colors in hex without # separated by comma to use as the gradient")

	var loopCount uint
	flag.UintVar(&loopCount, "loop_count", 1, "The number of times ot loop through thr GIF or the number of frames to show")

	var static bool
	flag.BoolVar(&static, "static", false, "Whether it's a static image (JPG/PNG) or not")

	var delay uint
	flag.UintVar(&delay, "delay", 0, "The delay between frames")

	var quantizer string
	flag.StringVar(&quantizer, "quantizer", "mediancut", "quantizer algorithm to use")

	flag.Parse()

	if threads < 1 {
		fmt.Println("Thread count must be at least 1")
		os.Exit(1)
	}

	colors, err := parseGradientColors(gradientColors)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	if loopCount < 1 {
		fmt.Println("Loop count must be at least 1")
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

	var img *gif.GIF

	if static {
		staticImg, format, err := image.Decode(file)
		if err != nil {
			fmt.Println("Error decoding static image: ", err)
			os.Exit(1)
		}
		img, err = staticTransform(staticImg, format, quantizer, delay)
	} else {
		img, err = gif.DecodeAll(file)
	}

	if err != nil {
		fmt.Println("Error decoding: ", err)
		os.Exit(1)
	}
	file.Close()

	frameCount := uint(len(img.Image)) * loopCount
	newFrames := make([]*image.Paletted, frameCount)
	for i := range newFrames {
		originalFrame := img.Image[i%len(img.Image)]
		newPalette := make([]color.Color, len(originalFrame.Palette))
		copy(newPalette, originalFrame.Palette)
		newFrames[i] = image.NewPaletted(originalFrame.Bounds(), newPalette)
	}

	gradient := newGradient(colors, true)
	overlayColors := gradient.generate(frameCount)

	framesPerThread := frameCount/threads + 1
	ch := make(chan uint)
	barrier := uint(0)

	for i := 0; i < int(threads); i++ {
		go func(base int, normalizedFrameIndex int) {
			for processed := 0; processed < int(framesPerThread); processed++ {
				frameIndex := base*int(framesPerThread) + processed

				if frameIndex >= int(frameCount) {
					break
				}

				if normalizedFrameIndex >= len(img.Image) {
					normalizedFrameIndex = 0
				}

				// do actual work in here
				prepareFrame(
					img.Image[normalizedFrameIndex],
					newFrames[frameIndex],
					overlayColors[frameIndex],
				)
				normalizedFrameIndex++
			}

			// thread is done
			ch <- 1
		}(i, i*int(framesPerThread)%len(img.Image))
	}

	// wait for all threads to synchronize
	for barrier != threads {
		barrier += <-ch
	}

	newDelay := make([]int, len(newFrames))
	// overwrite the delay if one is provided, otherwise use default
	for i := range newDelay {
		if delay == 0 {
			newDelay[i] = img.Delay[i%len(img.Delay)]
		} else {
			newDelay[i] = int(delay)
		}
	}

	newDisposal := make([]byte, len(newFrames))
	for i := range newDelay {
		newDisposal[i] = img.Disposal[i%len(img.Disposal)]
	}

	img.Image = newFrames
	img.Delay = newDelay
	img.Disposal = newDisposal

	file, err = os.OpenFile(output, os.O_RDWR|os.O_CREATE, 0644)
	if err != nil {
		fmt.Println("Error opening file: ", err)
		os.Exit(1)
	}

	img.Config.ColorModel = nil
	img.BackgroundIndex = 0

	err = gif.EncodeAll(file, img)
	if err != nil {
		fmt.Println("Error encoding image: ", err)
		os.Exit(1)
	}
	file.Close()
}
