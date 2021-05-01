const loader = () => {
    const WIDTH = 160;
    const HEIGHT = 144;

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    fetch('xpto.wasm')
        .then((res) => res.arrayBuffer())
        .then((ab) => WebAssembly.instantiate(ab))
        .then(({ instance }) => {
            instance.exports.memory.grow(100);              // make memory big enough
            // instance.exports.colorize(WIDTH, HEIGHT);
            instance.exports.initial_frame(WIDTH, HEIGHT);

            const data = new Uint8ClampedArray(instance.exports.memory.buffer, 0, WIDTH * HEIGHT * 4)
            const imageData = new ImageData(data, WIDTH, HEIGHT);

            ctx.putImageData(imageData, 0, 0)

        })
}

loader()
