Produces tables of ordered ranges of unicode characters

The files are fetched from:

https://www.unicode.org/Public/UCD/latest/ucd/

This call:
```rust
print_table([table::extract(&categs, |n| matches!(n, "Nl"))], [], "letter_number");
```

Produces this code:
```c
#define UCD_LEN_LETTER_NUMBER 12
static struct unicode_range ucd_table_letter_number[12] = {
    {0x000016ee, 0x000016f0},
    {0x00002160, 0x00002182},
    {0x00002185, 0x00002188},
    {0x00003007, 0x00003007},
    {0x00003021, 0x00003029},
    {0x00003038, 0x0000303a},
    {0x0000a6e6, 0x0000a6ef},
    {0x00010140, 0x00010174},
    {0x00010341, 0x00010341},
    {0x0001034a, 0x0001034a},
    {0x000103d1, 0x000103d5},
    {0x00012400, 0x0001246e},
};
```
