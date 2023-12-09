(module
  (func (export "steps") (param $number i32) (result i32) (local $steps i32)
    (if (i32.le_s (local.get $number) (i32.const 0))
      (then (return (i32.const -1))))
  

    (loop $collatz ;; creates only label, without branching there is no loop per se.
      (if (i32.gt_s (local.get $number) (i32.const 1))
        (then  
          (if (i32.eqz (i32.rem_s (local.get $number) (i32.const 2))) 
              (then 
                (local.set $number (i32.div_s (local.get $number) (i32.const 2))))
              (else 
                (local.set $number (i32.add (i32.mul (local.get $number) (i32.const 3)) (i32.const 1)))))

    
          (local.set $steps (i32.add (local.get $steps) (i32.const 1)))

          br $collatz ;; loop to label
        )
      )
    )
    
    local.get $steps ;; put return value on stack
        
  )
)
