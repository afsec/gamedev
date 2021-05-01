"use strict"

const CANVAS_ELEMENT = document.querySelector('canvas#screen')
const MON_HW = CANVAS_ELEMENT.getContext('2d')
const MON_WIDTH = 160
const MON_HEIGHT = 144
const XPTO_BG = "black"
const XPTO_BG_DEBUG = "red"
const XPTO_FG = "white"

MON_HW.fillStyle = XPTO_BG
MON_HW.fillRect(0, 0, 320, 200)
// MON_HW.fillRect(0, 0, MON_WIDTH, MON_HEIGHT)

CANVAS_ELEMENT.style = "width: 640px; height:576px;"

document.querySelector('button#zoom-in').addEventListener('click', () => {
    console.log('zoom-in')
    CANVAS_ELEMENT.style = "width: 640px; height:576px;"
})

document.querySelector('button#zoom-out').addEventListener('click', () => {
    console.log('zoom-out')
    CANVAS_ELEMENT.style = ""
})


fetch("xpto.wasm", { cache: "no-cache" }).then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, {})
).then(results => {
    let module = {};
    let mod = results.instance;
    module.alloc = mod.exports.alloc;
    module.dealloc = mod.exports.dealloc;
    module.fill = mod.exports.fill;
    module.clear = mod.exports.clear;



    let byteSize = MON_WIDTH * MON_HEIGHT * 4;
    var pointer = module.alloc(byteSize);
    var buffer = new Uint8Array(mod.exports.memory.buffer, pointer, byteSize);

    var button = document.querySelector('button#reboot')

    const ctx = MON_HW

    var pointer = module.alloc(byteSize);

    var usub = new Uint8ClampedArray(mod.exports.memory.buffer, pointer, byteSize);
    var img = new ImageData(usub, MON_WIDTH, MON_HEIGHT);
    var running = false;

    var frame = 0;
    var running = false;
    function step(timestamp) {
        if (!running) return;

        frame = module.fill(pointer, MON_WIDTH, MON_HEIGHT, frame);
        ctx.putImageData(img, 0, 0)

        if (frame != 65536) {
            window.requestAnimationFrame(step);
        } else {
            button.innerText = "Restart";
            running = false;
        }
    }

    function clearCanvasAndRestart() {
        running = false;
        window.requestAnimationFrame(() => {
            MON_HW.fillStyle = XPTO_BG
            MON_HW.fillRect(0, 0, MON_WIDTH, MON_HEIGHT)
            module.clear(pointer, MON_WIDTH, MON_HEIGHT)
            frame = 0
            running = true
            window.requestAnimationFrame(step)
        })
    }

    button.addEventListener("click", function (e) {
        if (running) {
            button.innerText = "Reboot";
            running = false;;
        } else {
            button.innerText = "Pause";
            clearCanvasAndRestart();
        }
    });
});