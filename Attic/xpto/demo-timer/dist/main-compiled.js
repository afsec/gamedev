'use strict';var a=0,b={MONITOR_WIDTH:160,MONITOR_HEIGHT:50,MONITOR_OUTPUT:document.querySelector("canvas#screen").getContext("2d"),cpuId:!1,vgaId:!1,CLOCK_DEFAULT_REALTIME:15778368E5,CLOCK_REALTIME:0,CPU:{BOZOMIPS:!1,CYCLES_PER_SECOND:0,CLOCK_MONOTONIC:0}},d=14,e=!1;
const g=document.querySelector("button#power-off"),k=()=>{setTimeout(()=>{b.CYCLES_PER_SECOND=a;h(`Cycles per second: ${b.CYCLES_PER_SECOND}`);h(`Bozomips: ${(1E3-1E3/b.CYCLES_PER_SECOND).toFixed(2)}`)},1E3)},l=()=>{e?h("The machine is already On"):(e=!0,b.cpuId=setInterval(()=>{var c=Date.now();a+=1;c=Date.now()-c;console.log(`${a} - elapsed time: ${c}`)},100),k())},h=c=>{const f=b.MONITOR_OUTPUT;f.font="14px Monospace";f.fillStyle="white";f.fillText(c,2,d);d+=14};
document.querySelector("button#power-on").addEventListener("click",()=>{document.querySelector("#power-led").style.backgroundColor="green";l()});g.addEventListener("click",()=>{document.querySelector("#power-led").style.backgroundColor="black";e?(e=!1,clearInterval(b.cpuId),console.info(a),a=0):h("The machine is already Off")});
