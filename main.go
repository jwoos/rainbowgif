package main


import (
	"fmt"
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

	decoded, err := gif.DecodeAll(file)
	if (err != nil) {
		fmt.Println("Error decoding: ", err)
		os.Exit(1)
	}
	fmt.Println(decoded)


	for i, frame := range decoded.Image {
		frame := decode.Image[i]
		delay := decode.Delay[i]
	}
}
