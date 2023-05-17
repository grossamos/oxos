#file ./target/aarch64-unknown-none/debug/kernel
add-symbol-file ./kernel8.debug
target remote localhost:1234
break kernel_main
