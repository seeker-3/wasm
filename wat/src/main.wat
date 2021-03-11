;; (module
;;   (func $console-log (import "console" "log" ) (param i32))

;;   (func $fn (param $a1 i32) (param $a2 i32) (result i32)
;;     ;; i32.add(local.get $a1 (local.get $a2))
;;     local.get $a1
;;   )
;;   (func (export "fn") (result i32)
    
;;     (i32.add( i32.const 1 (i32.const 1) ))
;;     i32.const 1
    
;;     i32.const 1
;;     i32.const 1
;;     i32.const call $fn

;;     (local $res i32)
    

;;     call $fn (i32.const 1) (i32.const 1)
;;     local.set $res

;;     local.get $res
;;     call $console-log
;;   )
;; )

;; (module
;;   (memory 0 10)
;;   (import "x" "x" (func))
;;   (func (import "x" "y"))
;;   (global $g (mut i32))
;;   (func $fn0 (param i32 i32) (result i32) i32.const 1)
;;   (func $eat_byte (param i32))
;;   (func (param $p0 i32) (local $l0 i32)
;;     i32.const 1
;;     i32.const 1
;;     call $fn0
;;     call $eat_byte
;;   )
;;   (func (export "0"))
;;   (export "1" (func 0))

;; )



;; (module
;;    (global $g (import "js" "global") ( i32))
;;    (func (export "getGlobal") (result i32)
;;         (global.get $g))
;;    (func (export "incGlobal")
;;         (global.set $g
;;             (i32.add (global.get $g) (i32.const 1))))
;; )

(module
  (memory 1)
  (data (i32.const 0) "hi")
  (func)
  (func
    (param i32)
  )
)