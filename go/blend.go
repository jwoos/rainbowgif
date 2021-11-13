package main

/* Functions for different blend modes
 */

import (
	"github.com/lucasb-eyer/go-colorful"
)

// top layer over bottom - most common
func blendNormal(top colorful.Color, topAlpha float64, bottom colorful.Color, bottomAlpha float64) (colorful.Color, float64) {
	alphaDelta := (1 - topAlpha) * bottomAlpha

	alpha := alphaDelta + topAlpha
	red := (alphaDelta*bottom.R + topAlpha*top.R) / alpha
	green := (alphaDelta*bottom.G + topAlpha*top.G) / alpha
	blue := (alphaDelta*bottom.B + topAlpha*top.B) / alpha

	result := colorful.Color{R: red / 255, G: green / 255, B: blue / 255}

	return result.Clamped(), alpha
}

/* color blend
 * preserves the luma of the bottom
 * adopts the hue and chroma of the top
 */
func blendColor(top colorful.Color, bottom colorful.Color) colorful.Color {
	topHue, topChroma, _ := top.Hcl()
	_, _, bottomLuma := bottom.Hcl()

	result := colorful.Hcl(topHue, topChroma, bottomLuma)

	return result.Clamped()
}

/* color blend
 * preserves the chroma and luma of the bottom
 * adopts the hue of the top
 */
func blendHue(top colorful.Color, bottom colorful.Color) colorful.Color {
	topHue, _, _ := top.Hcl()
	_, bottomChroma, bottomLuma := bottom.Hcl()

	result := colorful.Hcl(topHue, bottomChroma, bottomLuma)

	return result.Clamped()
}
