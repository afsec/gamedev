window.Module = {}
var Sha1 = {
  digest: (str) => {
    let buf = newString(Module, str)
    let outptr = Module.digest(buf)
    let result = copyCStr(Module, outptr)
    Module.dealloc_str(buf)
    return result
  }
}

fetchAndInstantiate("./sha1-digest.wasm", {})
  .then(mod => {
    Module.alloc = mod.exports.alloc
    Module.dealloc = mod.exports.dealloc
    Module.dealloc_str = mod.exports.dealloc_str
    Module.digest = mod.exports.digest
    Module.memory = mod.exports.memory

    var input = document.getElementById("input")
    var output = document.getElementById("output")
    output.innerText = Sha1.digest(input.value)

    input.addEventListener("keyup", function (e) {
      output.innerText = Sha1.digest(input.value)
    })
  })
