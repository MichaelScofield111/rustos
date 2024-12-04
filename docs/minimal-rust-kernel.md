## introduction
```text
we create a minimal 64-bit rust kernel for the x86 architecture.
```

### the boot process
```text
it begins exeuting firmware code that loads into memory. Afterwards, it look for a bootable disk and starts booting the oeprating system kernel.
```

### BIOS Boot
```text
   1. loads BOIS from some special flash memory located on the motherboard.
   2. look for bootable disk(bootloader)
   3. bootloader, which is a 512-byte portion of executable code stored at the disk’s beginning. 

   The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, 
```

## A Minimal Kernel
`goal`
```text
Our goal is to create a disk image that prints a “Hello World!” to the screen when booted. We
```

### target specifications
```text
cargo supports different target systems through the --`target` parameter. \n
the target describes by so called target triple, which describes the `CPU` architecture, `vendor`, `OS` and `ABI` (Application Binary Interface). \n
For example, the x86_64-unknown-linux-gnu target triple describes a system with an x86_64 CPU, no clear vendor, and a Linux operating system with the GNU ABI.  \n

For our target system, however, we require some special configuration parameters (e.g. no underlying OS), so none of the existing target triples fits. Fortunately, Rust allows us to define our own target through a JSON file. For example, a JSON file that describes the x86_64-unknown-linux-gnu target looks like this:
```
```
{
    "llvm-target": "x86_64-unknown-linux-gnu",
    "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": "32",
    "os": "linux",
    "executables": true,
    "linker-flavor": "gcc",
    "pre-link-args": ["-m64"],
    "morestack": false
}
```