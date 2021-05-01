# The Mandelbrot Set

The foundations were now in place, so I could code something more interesting to look at. I chose the Mandelbrot Set because it uses a simple formula yet gives a spectacular result.

For more information on how the Mandelbrot set is calculated, check out the Wikipedia page, but essentially the code goes as follows:

- Iterate through every memory location.
- Calculate a corresponding real and imaginary (x and y) coordinate for the pixel.
- Repeatedly apply the Mandelbrot formula at that coordinate until either its value grows too large, or after a fixed number of iterations have occurred.
- Choose a colour from a predefined palette of 8 colours, depending on the number of iterations that occurred.

The colours for the image are chosen from a hard-coded palette of 8 colours, selected by taking the least significant 3 bits of the iteration count. The palette is defined using data expressions and stored in memory after the pixel data. A minor change is needed to main.js to add an extra page of memory for it:
```js
// Allocate some memory for the module. Initial size is specified in pages of 16 kiB.
// 16 pages for the pixel data, and an extra page for the palette data (defined in main.wat)
const memory = new WebAssembly.Memory( { initial: 17 } );

// the rest as before...
// ...
```

Here is the WAT file in all its glory. Rather than explain each instruction here, I have commented the code, so hopefully, you can see what it’s doing. The algorithm is the [escape-time algorithm from the Wikipedia page](https://en.wikipedia.org/wiki/Mandelbrot_set#Computer_drawings).

The WAT text format allows a more compact “s-expression” representation, which groups instructions together on a line using parentheses. I deliberately chose not to use it for this experiment, preferring to keep to a more traditional assembly style, but next time I think I will use the other style to improve readability.

Although there is a lot of code here, the essentials are the same as the previous examples. The only new things are some arithmetic and logic operations (`sub`, `mul`, `div`, and, `le`, `lt_u`), converting between integer and floating point types (`convert_u`), and using `data` to define the colour palette. I encourage you to consult the Web Assembly documentation to find out more about these instructions if you are interested.

So there you have it — a Mandelbrot set calculated using Web Assembly and drawn onto an HTML canvas. There’s a lot of scope for improvement, but I found it to be a fun and interesting exercise to introduce myself to Web Assembly. I hope you do too!

**Source:** (https://medium.com/@alexc73/programming-using-web-assembly-c4c73a4e09a9)
