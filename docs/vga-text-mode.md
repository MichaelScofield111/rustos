## introduction
```text
the vga text mode is a simple way to print text to the screen.
```
### The VGA Text Buffer
```text
The VGA text buffer is a two-dimensional array with 25 rows and  80 columns, witch is directly rendered to the screen. Each array entry describes a single screen character through the following format:
```
| (Bits) | (Value)          |
|--------|------------------|
| 0-7       | ASCII code point    |
| 8-11      | foreground render   |     
| 12-14     | background remder   |
| 15        | blink               |

```text
The VGA text buffer is accessible via memory-mapped I/O to the address `0xb8000`. this means that reads and writes to that addrsss don't access the RAM but directly access the text buffer on the VGA hardware.
```
