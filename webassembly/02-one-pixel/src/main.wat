(module
  (import "js" "mem" (memory 16))
  (func $setOnePixel
    i32.const 0          ;; memory address 0 = first pixel
    i32.const 0xff880088 ;; RGBA colour little-endian = 0xAABBGGRR
    i32.store            ;; write to memory
  )
  (export "run" (func $setOnePixel))
)
