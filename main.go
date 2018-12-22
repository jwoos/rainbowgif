package main


import (
	"fmt"
	//"image"
	//"image/color"
	"image/gif"
	//"io/ioutil"
	"os"

	//"github.com/lucasb-eyer/go-colorful"
)


func main() {
	file, err := os.Open("test.gif")
	if (err != nil) {
		fmt.Println("Error opening file: ", err)
		os.Exit(1)
	}

	originalImage, err := gif.DecodeAll(file)
	if (err != nil) {
		fmt.Println("Error decoding: ", err)
		os.Exit(1)
	}

	frameCount := len(originalImage.Image)
	fmt.Println(frameCount)

	/* frame is just `*image.Paletted`
	 * `color.Palette` is just `[]color.Color`
	 * `[]color.Color` is an interface implementing `RGBA()`
	 */
	/*
	 *for _, frame := range originalImage.Image {
	 *    originalPalette := frame.Palette
	 *    newPalette := make([]color.Color, len(frame.Palette))
	 *}
	 */

	/*
	 *transformedImage := gif.GIF{
	 *    Image: make([]*image.Paletted, 0, len(originalImage.Image)),
	 *    Delay: make([]int, len(originalImage.Delay)),
	 *    LoopCount: originalImage.LoopCount,
	 *    Disposal: originalImage.Disposal,
	 *    Config: originalImage.Config,
	 *    BackgroundIndex: originalImage.BackgroundIndex,
	 *}
	 */
}
