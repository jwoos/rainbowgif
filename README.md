# Rainbow GIF
## What?
This is a program to read in images and overlay colors over the frames to create a rainbow effect. The tool currently supports GIFs as well as JPGs and PNGs.

Before:
![Before](images/fidget_spinner.gif)

After:
![After](images/fidget_spinner_rainbow.gif)

## Usage
Clone it and assuming you have Go a version greater than or equal to 1.11, you should just be able to do a `go mod download` to download all the modules and then `go build`. This should output a binary in the directory. Run it with by doing `./rainbowgif <input> <output>`.

### Options
- `threads`: The number of goroutines to use when processing the GIF
- `gradient`: The list of colors to use as the overlay. When omitted, it will default to ROYGBV.
- `loop_count`: Defaults to 1.
  - For GIF: The number of times to loop over the GIF. The output GIF will be `loop_count` times longer.
  - For static images (JPG, PNG): The number of frames to create for the resulting GIF. The output will be `loop_count` frames long.
- `static`: Indicate whether the input is a static image. Defaults to false. Hopefully this can be removed in the future.
- `quantizer`: Only used with `static` on. This will choose which quantizer to use.
- `delay`: This sets the delay between frames in 100ths of a second

## Technical Detail
This makes use of https://github.com/lucasb-eyer/go-colorful - this library saved me a lot of travel since the standard color library doesn't cover all this.

I'll be explaining how the colors are generated and used below.

Before we begin, I'll just explain some variables even though I'll be describing and naming them as I go.

| Prefix | Meaning |
| ------ | ------- |
| `n_` | Number of or count |
| `i_` | Index |
| `p_` | Position |
| `dp_` | Delta position |

| Base | Meaning |
| ------ | ------- |
| `f` | Frame of GIF |
| `ci` | Input color |
| `cg` | Gradient generated color |

### Gradient
The gradient can be initialized with varying number of colors (`n_ci`), but for the purposes of this script I chose seven: ROYGBV and a seventh color close to red. Without this last color, the looping won't be as smooth as there will be a suddent transition from the last color to the first color.

This is what the input colors would look like:
```
| R | O | Y | G | B | V | R' |
```

However regardless of `n_ci`, there can be any number of generated colors (`n_cg`). `n_cg` will depend only on how many frames the input GIF has (`n_f`). Given a 10 frame GIF, the program will generate 10 different colors to apply to each frame - in short, `n_cg == n_f`. All output colors will be either an input color or two input colors blended in HCL mode.

Colors are calculated using `n_f`, the index of the frame (`i_f`), positions of the frame (`p_f`), and position of the generated color (`p_cg`). Position is represented as a number between 0 and 1. It's meant to show where a color is in the gradient.

Given a 10 frame GIF, the position would look like the following (top is index and bottom is position):

| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
| - | - | - | - | - | - | - | - | - | - |
| 0 | 0.1 | 0.2 | 0.3 | 0.4 | 0.5 | 0.6 | 0.7 | 0.8 | 0.9 | 1 |

Given the previous input colors, the positions would look like the following (top is index and bottom is position):

| 0 | 1 | 2 | 3 | 4 | 5 | 6 |
| - | - | - | - | - | - | - |
| 0 | 0.1667 | 0.3333 | 0.5 | 0.6667 | 0.8333 | 1 |

Positions for frames are found by doing `i_f / (n_f - 1)` - the subtraction by one is important due to zero indexing. It's clear here that the delta between the positions (`dp_f`) is `1 / (n_f - 1)`. Similarly, positions for the input colors are found by doing `i_ci / (n_ci - 1)` and the delta (`dp_ci`)  is `1 / (n_ci - 1)`. In some cases, the frame's position will line up with the color's position. However in the majority of the cases, it won't. For those cases, the relative position can be found between the colors.

Before moving on, we can intuite this. Using the previous examples, we know that frame 0 has `p_f = 0` and that correlates directly to `p_cg = 0` so frame 0 will be using colors 0. Frame 1's position is 0.1 and that's nowhere to be found on the gradient color position list. However we can see that it's between 0 and 0.1667, so we can say that it's between colors 0 and 1.

The actual calculation is basically just that - first `p_f` is divided by `dp_ci`. This number is then floored and this can be considered the lower index. It is then incremented and considered the higher index. There we have our two colors using the same idea as described above. Of course, this won't work if the lower index happens to be last index. In that case, the higher index will be equal to the lower index.

Using frame 1 again as an example we know that its position is 0.1 and `dp_ci` is `0.1667`. Doing `floor(0.1 / 0.1667)` yields 0, so we know that colors 0 and 1 are the two colors to blend.

Now that we know which two colors to blend - we just blend them right? Not quite - since we care about just how close to each color the position really is. Let's call the two colors c1 and c2. To use c1's position as the base and c2's position as the end, we simply have to subtract both by c1's position. This will leave c1's position as 0 and c2's position as some number (it doesn't matter which). We also subtract the input position by c1's original position to get the relative position. This is then divided by c2's new position, which yields a number between zero and one. This number is then used to get the color at that position using the colorful library's HCL blending function.

This step is repeated for all of the frames, yield `n_cg` colors.

### Blend
The blending I'm talking about here is blending mode as used in image editors such as PhotoShop. I opted to use PhotoShop's color blending mode, which works in HCL colorspace. HCL is hue, chroma, and luma. The color blending mode preserves the bottom layer's luma while adopting the top layer's hue and chroma. This deals with the issue of having a solid color just being overlayed on top. In addition, any 0 alpha pixel will be ignored preserving the original transparency.
