// loader.js

const DEBUG_FRONTEND = true

var HW_PERIPHERALS = {
    "MONITOR_WIDTH": 160,
    "MONITOR_HEIGHT": 50,
    "MONITOR_OUTPUT": document.querySelector('canvas#screen').getContext("2d"),
    "cpuId": false,
    "vgaId": false,
    "CLOCK_DEFAULT_REALTIME": 1577836800000, // Wednesday, January 1, 2020 12:00:00 AM GMT
    "CLOCK_REALTIME": 0, // Must be GMT
    "CPU": {
        "BOZOMIPS": false,
        "CYCLES_PER_SECOND": 0,
        "CLOCK_MONOTONIC": 0,
    }
}
const TERMINAL_FONT_SIZE = 10
const TERMINAL_FONT_SPACING = 4
const TERMINAL_FONT_PADDING_Y = TERMINAL_FONT_SIZE + TERMINAL_FONT_SPACING


const CURSOR_X = 2;

var CURSOR_Y = TERMINAL_FONT_PADDING_Y;



var VGA_FPS = 20 // Frames per second

var HW_POWER_SW = false

const HW_JIFFY = 100 // 1 ms

// var BOZOMIPS_COUNTER = 0


const HW_POWER_ON = document.querySelector('button#power-on')
const HW_POWER_OFF = document.querySelector('button#power-off')

const VGA_INTERFACE = document.querySelector('p#monitor-output')

const calculateBozomips = () => {
    setTimeout(() => {
        HW_PERIPHERALS["CYCLES_PER_SECOND"] = cycles
        writeTerminal(`Cycles per second: ${HW_PERIPHERALS["CYCLES_PER_SECOND"]}`)
        const cyclesPerSecond = HW_PERIPHERALS["CYCLES_PER_SECOND"]
        const bozoMips = 1000 - (1000 / cyclesPerSecond) // Minus 10 is human readable trick
        writeTerminal(`Bozomips: ${(bozoMips).toFixed(2)}`)
    }, 1000)
}


const powerOn = () => {
    if (HW_POWER_SW) {
        writeTerminal('The machine is already On')
    } else {
        HW_POWER_SW = true
        HW_PERIPHERALS["cpuId"] = setInterval(() => {
            const result = cpuRunCycle()
            console.log(`${cycles} - elapsed time: ${result}`)
        }, HW_JIFFY)
        calculateBozomips()
        // HW_PERIPHERALS.vgaId = setInterval(myVga, 1000 / VGA_FPS)
    }
}


const powerOff = () => {
    if (HW_POWER_SW) {
        HW_POWER_SW = false
        // VGA_INTERFACE.innerHTML = ''
        clearInterval(HW_PERIPHERALS["cpuId"])
        // clearInterval(HW_PERIPHERALS.vgaId)
        if (DEBUG_FRONTEND) {
            console.info(cycles)
        }
        cycles = 0
    } else {
        writeTerminal('The machine is already Off')
    }
}



const myVga = () => {
    VGA_INTERFACE.innerHTML = cycles
}

const writeTerminal = (message) => {
    const ctx = HW_PERIPHERALS["MONITOR_OUTPUT"]
    ctx.font = `${TERMINAL_FONT_PADDING_Y}px Monospace`
    ctx.fillStyle = "white"
    ctx.fillText(message, CURSOR_X, CURSOR_Y)
    CURSOR_Y = CURSOR_Y + TERMINAL_FONT_PADDING_Y
    // console.log(message)

}


HW_POWER_ON.addEventListener('click', () => {
    document.querySelector('#power-led').style.backgroundColor = "green"
    // writeTerminal('powerOn()')
    powerOn()
})

HW_POWER_OFF.addEventListener('click', () => {
    document.querySelector('#power-led').style.backgroundColor = "black"
    // writeTerminal('powerOff()')
    powerOff()
})



