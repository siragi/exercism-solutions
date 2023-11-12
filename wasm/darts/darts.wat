(module
  (func (export "score") (param $x f32) (param $y f32) (result i32)
    (local $radius f32)
    ;; stacky way (like Forth)
    local.get $x
    local.get $x
    f32.mul
  
    local.get $y
    local.get $y
    f32.mul
  
    f32.add
    f32.sqrt
    ;; Or lispy
    ;; (f32.sqrt (f32.add
    ;;     (f32.mul (local.get $x) (local.get $x))
    ;;     (f32.mul (local.get $y) (local.get $y))))

    local.tee $radius ;; (store and stack together https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_tee 

    f32.const 1
    f32.le
    if (result i32)  
      i32.const 10
    else 
      (f32.le (local.get $radius) (f32.const 5))
      if (result i32) 
        i32.const 5
      else 
        (f32.le (local.get $radius) (f32.const 10)) 
        if (result i32)  
          i32.const 1
        else 
          i32.const 0
        end 
      end 
    end

    ;; lispy way: (if (predicate) (then ) (else )
    ;; (if (f32.le (local.get $radius) (f32.const 1))
    ;;     (then (return (i32.const 10))))
    ;; (if (f32.le (local.get $radius) (f32.const 5))
    ;;     (then (return (i32.const 5))))
    ;; (if (f32.le (local.get $radius) (f32.const 10))
    ;;     (then (return (i32.const 1)))
    ;;     (else (return (i32.const 0))))
    ;; unreachable
  )
)
