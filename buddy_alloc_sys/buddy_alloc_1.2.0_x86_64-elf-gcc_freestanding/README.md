# buddy_alloc binaries

For using in x86_64 kernel
#### x86_64-elf tools
https://newos.org/toolchains

# How to compile library and generate binding
This is not something that everyone needs to do, this is a hint to me.

#### buddy_alloc_1.2.0_debug.o
`x86_64-elf-gcc -ffreestanding -mno-red-zone -lgcc -m64 -nostdlib -fno-pic -fno-builtin -fno-stack-protector -fno-omit-frame-pointer -g -Og -c buddy_alloc_1.2.0.c -o buddy_alloc_1.2.0_debug.o`

#### buddy_alloc_1.2.0_debug.a
`x86_64-elf-ar rcs buddy_alloc_1.2.0_debug.a buddy_alloc_1.2.0_debug.o`

#### buddy_alloc_1.2.0_release.o
`x86_64-elf-gcc -ffreestanding -mno-red-zone -lgcc -m64 -nostdlib -fno-pic -fno-builtin -fno-stack-protector -O3 -c buddy_alloc_1.2.0.c -o buddy_alloc_1.2.0_release.o`

#### buddy_alloc_1.2.0_release.a
`x86_64-elf-ar rcs buddy_alloc_1.2.0_release.a buddy_alloc_1.2.0_release.o`

### Bindgen (buddy_alloc/src/lib.rs)

```
(echo '#![no_std]
#![allow(warnings)]'; \
bindgen --use-core --allowlist-item buddy_.* --allowlist-item BUDDY_.* buddy_alloc_1.2.0.c.h -- \
    --target=x86_64-elf -ffreestanding -fno-builtin -nostdinc \
    -isystem /path/to/your/x86_64-elf-13.2.0-Linux-x86_64/bin/../lib/gcc/x86_64-elf/13.2.0/include \
    -isystem /path/to/your/x86_64-elf-13.2.0-Linux-x86_64/bin/../lib/gcc/x86_64-elf/13.2.0/include-fixed) > ../src/lib.rs
```

Prints x86_64-elf-gcc include path
```
echo | x86_64-elf-gcc -E -Wp,-v -
```