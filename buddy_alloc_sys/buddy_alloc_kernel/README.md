# buddy_alloc binaries

For using in x86_64 kernel
#### x86_64-elf tools
https://newos.org/toolchains

# How to compile library and generate binding
This is not something that everyone needs to do, this is a hint to me.

#### buddy_alloc_debug_kernel.o
```
x86_64-elf-gcc -ffreestanding -mno-red-zone -lgcc -m64 -nostdlib -fpic -fno-builtin -fno-stack-protector -fno-omit-frame-pointer -mcmodel=large \
    -mno-mmx -mno-sse -mno-sse2 -mno-sse3 -mno-ssse3 -mno-sse4 -mno-avx -mno-avx2 -mno-fma -msoft-float \
    -g -Og -c buddy_alloc.c -o buddy_alloc_debug_kernel.o
```

#### libbuddy_alloc_debug_kernel.a
`x86_64-elf-ar rcs libbuddy_alloc_debug_kernel.a buddy_alloc_debug_kernel.o`

#### buddy_alloc_release_kernel.o
```
x86_64-elf-gcc -ffreestanding -mno-red-zone -lgcc -m64 -nostdlib -fpic -fno-builtin -fno-stack-protector -mcmodel=large \
    -mno-mmx -mno-sse -mno-sse2 -mno-sse3 -mno-ssse3 -mno-sse4 -mno-avx -mno-avx2 -mno-fma -msoft-float \
    -O3 -c buddy_alloc.c -o buddy_alloc_release_kernel.o
```

#### libbuddy_alloc_release_kernel.a
`x86_64-elf-ar rcs libbuddy_alloc_release_kernel.a buddy_alloc_release_kernel.o`

### Bindgen (buddy_alloc/src/lib.rs)

```
(echo '#![no_std]
#![allow(warnings)]'; \
bindgen --use-core --allowlist-item buddy_.* --allowlist-item BUDDY_.* buddy_alloc.h -- \
    --target=x86_64-elf -ffreestanding -fno-builtin -nostdinc \
    -isystem /path/to/your/x86_64-elf-13.2.0-Linux-x86_64/bin/../lib/gcc/x86_64-elf/13.2.0/include \
    -isystem /path/to/your/x86_64-elf-13.2.0-Linux-x86_64/bin/../lib/gcc/x86_64-elf/13.2.0/include-fixed) > ../src/lib.rs
```

Prints x86_64-elf-gcc include path
```
echo | x86_64-elf-gcc -E -Wp,-v -
```
