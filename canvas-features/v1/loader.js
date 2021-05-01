"use strict"

const CANVAS_ELEMENT = document.querySelector('canvas#screen')

const HW_MON = CANVAS_ELEMENT.getContext('2d') // Retorna o objeto canvas que permite desenhar
const HW_MON_WIDTH = 160
const HW_MON_HEIGHT = 50

var HW_POWER_ON = false

var ZOOM = 3

// CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;`

CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;` // Original size
// CANVAS_ELEMENT.style = "" // Original size

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


const drawPixel = (x, y) => {
    if (HW_POWER_ON) {
        HW_MON.fillStyle = "white" // CSS color notation
        HW_MON.fillRect(x, y, 1, 1)
    } else {
        console.error('XPTO computer is off. You must run "powerOn()" before send that command.')
    }

}

///////////////

const powerOn = () => {
    HW_POWER_ON = true
    console.warn('XPTO computer is on. Now you can input these commands: "clearScreen()" "drawPixel()".')
    clearScreen()
}

const clearScreen = () => {
    if (HW_POWER_ON) {
        HW_MON.fillStyle = "black" // CSS color notation
        HW_MON.fillRect(0, 0, HW_MON_WIDTH, HW_MON_HEIGHT)
    } else {
        console.error('XPTO computer is off. You must run "powerOn()" before send that command.')
    }
}

////////////////////////


const main = () => {
    console.error('XPTO computer is off. First of all you must run "powerOn()".')
    powerOn()

}

main()