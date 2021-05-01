"use strict"

const CANVAS_ELEMENT = document.querySelector('canvas#screen')

const HW_MON = CANVAS_ELEMENT.getContext('2d') // Retorna o objeto canvas que permite desenhar
const HW_MON_WIDTH = 160
const HW_MON_HEIGHT = 50

const CENTER_X = Math.floor(HW_MON_WIDTH / 2);
const CENTER_Y = Math.floor(HW_MON_HEIGHT / 2);

var HW_POWER_ON = false

var ZOOM = 3

// CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;`

CANVAS_ELEMENT.style = `width: ${HW_MON_WIDTH * ZOOM}px; height: ${HW_MON_HEIGHT * ZOOM}px;` // Original size
// CANVAS_ELEMENT.style = "" // Original size

// START BUTTONS 

document.querySelector('button#clear').addEventListener('click', () => clearScreen())
document.querySelector('button#show-border').addEventListener('click', () => drawBorder())
document.querySelector('button#circle').addEventListener('click', () => drawCircle(CENTER_X, CENTER_Y, 24))
document.querySelector('button#target').addEventListener('click', () => drawTarget(CENTER_X, CENTER_Y))
document.querySelector('button#animation').addEventListener('click', () => drawAnimation(CENTER_X, CENTER_Y))
//
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

// END BUTTONS 

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


const drawBorder = () => {
    for (let i = 0; i < 50; i++) {
        drawPixel(0, i)
    }

    for (let i = 0; i < 50; i++) {
        drawPixel(159, i)
    }

    for (let i = 0; i < 160; i++) {
        drawPixel(i, 0)
    }

    for (let i = 0; i < 160; i++) {
        drawPixel(i, 49)
    }

}

const drawTarget = (x,y) => {
    for (let i = 0; i < 10; i++) {
        setTimeout(() => {
            drawPixel(x + i, y)
            drawPixel(x - i, y)
            drawPixel(x, y + i)
            drawPixel(x, y - i)
        }, (100 * i))

    }
}

/*
const power2 = (x) => x * x
// Better use pow() instead
const sqrt = (x) => {
    // Better use Math.sqrt() instead
    var done = false
    let i = 0
    let result = null
    while (done === false) {
        i++
        const pow = power2(i)
        if (pow === x) {
            done = true
            result = i
        } else {
            if (pow > x) {
                done = true
                console.info(`power2(${i}) > x`, pow, x)
                if ((pow - power2((i - 1))) < (pow - power2(i))) {
                    const q = power2(i - 1)
                    result = (x + q) / (2 * sqrt(q))
                } else {
                    const q = power2(i)
                    result = (x + q) / (2 * sqrt(q))
                }
            }
            // Tried 10 times without success
            if (i > 19) {
                done = true
                // This is the last iteration
                result = false
            }
        }

    }
    return result
}
*/
const drawCircle = (x, y, r) => {
    drawPixel(x, y) // Center of the circle
    // Draw from Up to Right
    for (let i = 0; i < (r + 1); i++) {
        const h = i
        const v = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(h, 2)))
        drawPixel(x + i, y - v)
    }

    // Draw from Up to Left
    for (let i = 0; i < (r + 1); i++) {
        const h = i
        const v = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(h, 2)))
        drawPixel(x - i, y - v)
    }

    // Draw from Down to Left
    for (let i = 0; i < (r + 1); i++) {
        const h = i
        const v = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(h, 2)))
        drawPixel(x - i, y + v)
    }

    // Draw from Down to Right
    for (let i = 0; i < (r + 1); i++) {
        const h = i
        const v = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(h, 2)))
        drawPixel(x + i, y + v)
    }

    // Draw from Left to Up
    for (let i = 0; i < (r + 1); i++) {
        const v = i
        const h = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(v, 2)))
        drawPixel(x - h, y - i)
    }


    // Draw from Left to Down
    for (let i = 0; i < (r + 1); i++) {
        const v = i
        const h = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(v, 2)))
        drawPixel(x - h, y + i)
    }

    // Draw from Right to Down
    for (let i = 0; i < (r + 1); i++) {
        const v = i
        const h = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(v, 2)))
        drawPixel(x + h, y + i)
    }

    // Draw from Right to Up
    for (let i = 0; i < (r + 1); i++) {
        const v = i
        const h = Math.round(Math.sqrt(Math.pow(r, 2) - Math.pow(v, 2)))
        drawPixel(x + h, y - i)
    }

    // drawPixel((x - r), y)  // Left pixel
    // drawPixel(x, (y + r))  // Down pixel
}

//////////////////////////////////
// const procId = setInterval(() => drawPixel(Math.floor(Math.random() * ((160-0)+0)),Math.floor(Math.random() * ((50 - 0)))),10)

const drawAnimation = (x,y) => {
    for (let i = 0; i < 25; i = i + 2) {
         setTimeout(() => {
            drawCircle(x,y, i)
        }, 100 * i)
    }
    setTimeout(clearScreen, 25 * 100)
}


const main = () => {
    console.error('XPTO computer is off. First of all you must run "powerOn()".')
    powerOn()
    // anim(2)

}

main()