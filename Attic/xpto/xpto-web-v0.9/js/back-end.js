"use strict"

// back-end.js

var cycles = 0

const DEBUG_BACKEND = true

const cpuRunCycle = () => {
    // * CPU Main Loop
    const startCycleTime = Date.now()
    //

    cycles += 1 // Increment Cycle

    //
    const elapsedCycleTime = Date.now() - startCycleTime
    return elapsedCycleTime
}




// * Make Registers
// ? How put Instructions to memory. (Input devices)
// ? How get Instructions from memory. (Output devices)
// ? How to calculate CPU Clock.

