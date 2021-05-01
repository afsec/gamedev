"use strict"

const CANVAS_ELEMENT = document.querySelector("canvas#demo-canvas")

const HW_MON_WIDTH = 160
const HW_MON_HEIGHT = 50

CANVAS_ELEMENT.width = HW_MON_WIDTH
CANVAS_ELEMENT.height = HW_MON_HEIGHT

var ZOOM = 4

var cycles = 0

CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;`

document.querySelector('button#zoom-original').addEventListener('click', () => {
  ZOOM = 1
  console.log('zoom-original')
  CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;`
})

document.querySelector('button#zoom-in').addEventListener('click', () => {
  console.log('zoom-in')
  ZOOM = ZOOM + 1
  CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;`
})

document.querySelector('button#zoom-out').addEventListener('click', () => {
  console.log('zoom-out')
  ZOOM = ZOOM - 1
  CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;`
})


async function init() {
  const { instance } = await WebAssembly.instantiateStreaming(
    fetch("./bare_metal_wasm.wasm")
  )

  const buffer_address = instance.exports.FRAME_BUFFER.value

  const image = new ImageData(
    new Uint8ClampedArray(
      instance.exports.memory.buffer,
      buffer_address,
      4 * HW_MON_WIDTH * HW_MON_HEIGHT,
    ),
    HW_MON_WIDTH,
  )

  const ctx = CANVAS_ELEMENT.getContext("2d")

  // STATIC
  // instance.exports.go()
  // ctx.putImageData(image, 0, 0)

  // ANIMATED
  const render = () => {
    console.log("cycle")
    instance.exports.go()
    ctx.putImageData(image, 0, 0)
  }
  render()
  setInterval(render,29)

}

init()
