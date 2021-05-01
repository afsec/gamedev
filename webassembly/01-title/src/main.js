"use strict"
fetch('main.wasm')
  .then(response =>
    response.arrayBuffer()
  )
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(results => {
    const wasm = results.instance;
    document.querySelector('span#container').textContent = wasm.exports.foursec();
    console.info(wasm.exports.myrandom())
  })
  .catch(console.error)

