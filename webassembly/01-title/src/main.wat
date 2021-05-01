
(module ;; Module declaring
  (func ;; Function Declaring
    (result i32) ;; What this functions returns
    i32.const 12
    i64.const 12
    f32.const 123.45
    f64.const 123.45 
    i32.const 1234 ;; Returns just a i32 Integer
  )
  ( func
    (result i32)

    i32.const 4
  )
  (export "foursec" (func 0)) ;; Export this function as a name "foursec" 
  (export "myrandom" (func 1)) ;; Export this function as a name "myrandom" 
)
