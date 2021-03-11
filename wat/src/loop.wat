(module
  (import "console" "log" (func $log (param i32)))
  (func $main
   (local $count i32)

    i32.const 0
    local.set $count
    loop $loop
      local.get $count
      call $log

      local.get $count
      i32.const 1
      i32.add
      local.set $count

      local.get $count
      i32.const 100
      i32.ne
      br_if $loop
    end
  )
  (start $main)
)