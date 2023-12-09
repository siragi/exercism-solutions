(module
  (func (export "squareRoot") (param $radicand i32) (result i32)
    (local $estimate i32) (local $complement i32)

    (local.set $estimate (i32.trunc_f32_s (call $sqrtEstimate (local.get $radicand)))) 
    ;;(local.set $estimate (i32.const 1)) ;; would also work, but that would have been too simple :-)

    (loop $heron
      (local.set $complement (i32.div_s (local.get $radicand) (local.get $estimate)))

      (if (i32.ne (local.get $estimate) (local.get $complement))
        (then
          (local.set $estimate (i32.add (local.get $estimate) (i32.const 1)))
          br $heron
        )
      )
    )
    local.get $complement
  )

  (func $sqrtEstimate (param $radicand i32) (result f32)
    (local $estimate f32) ;; to get the scientific (binary) form:     a * 2^(2n) of the radicand 
                          ;; where 0.1b <= a <= 10b (betw 0.5 and 2) 
                          ;; square root estimate is then:      sqrt(a) * 2^n which can be linearly approximated in that interval
                          ;; by (0.485 + 0.485*a) * 2^n, which is almost: (0.5 + 0.5*a) * 2^n
    (local $bits i32)
    (local.set $bits (i32.sub (i32.const 32) (i32.clz (local.get $radicand)))) ;; count leading zeros (clz) of the binary representation (i32 has 32bits and find out how many meaningful bits we have) 

    (if (i32.and (i32.const 1) (local.get $bits)) ;; bitwise and 00000001 -> 1 then $bits are odd (meaning the last bit of $bits was 1)
      (then  ;; representation as 1b * 2^(2*(($bits-1)/2)) => and sqrt approx is: (.5 + .5*1) * 2^((bits-1)/2)
        (local.set $estimate 
          (f32.convert_i32_s (i32.shl (i32.const 1) ;; shl 1 by n => 2^n 
                   (i32.shr_s  (i32.sub (local.get $bits) (i32.const 1)) (i32.const 1))))) ;; n => shr ($bits-1) by 1  => ($bits-1)/2 
      )
      (else                                   ;; representation as 0.1b * 2^(2*($bits/2)) and sqrt approx is: 0.75 * 2^halfbits
        (local.set $estimate (f32.mul
          (f32.const 0.75) 
          (f32.convert_i32_s (i32.shl (i32.const 1) ;; shl 1 by n => 2^n 
                   (i32.shr_s (local.get $bits) (i32.const 1))))))) ;; n => shr $bits by 1  => $bits/2 
      
    )
    local.get $estimate
  )
)

