# devnotes

* Most of the BT ROM is compressed and needs to be decompressed to be modifyable
  * Compression used: gzip on `-9` compression (thanks to N64-Tools for this info)
  * The files within the BT ROM had their gzip header stripped. For gzip to recognized the compressed file, prepend the following header: `1F8B0800 00000000 0203`
  * It seems like gzip also adds a footer to compressed files. These aren't present in the BT ROM either, but gzip can still decompress the files (although it gives an "unexpected EOF" error; just ignore it ...)
  * The compressed files in the ROM have a custom 2-byte header. To calculate the 2-bytes that need to be here (thanks to N64-Tools for this info):
    * byte1 = (((unsigned short) ((float) size_of_file / (float) 0x10)) >> 8) && 0xFF
    * byte2 =  ((unsigned short) ((float) size_of_file / (float) 0x10)) && 0xFF
  * The compressed files in the ROM are right-padded to a full 8 bytes (1 dword) via 0xAA's. These may have to be stripped before decompressing via gzip, and may have to be added before compressing. (This might be different depending on asm or non-asm files?)
