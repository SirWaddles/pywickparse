# pywickparse

Python Bindings for the JohnWickParse library

## Extracting

The `PakExtractor` class is used to extract files from a .pak, and it's use is pretty basic.

```python
import pywickparse

# Make a new PakExtractor by specifying the pak file and the key (in hex)
extractor = pywickparse.PakExtractor("pakchunk.pak", "0000000000000000000000000000000000000000000000000000000000000000")
# get_file_list returns a List of filenames as strings, as they're represented in the pak
files = extractor.get_file_list()
# Use the index of the above list, to retrieve the bytes of a file
file = extractor.get_file(0)
print(files[0])
handle = open("./test", "wb")
handle.write(file)
handle.close()
```

## Parsing Files

The `read_asset(path: string)` function is the main way to parse an asset file.

The path parameter **should not** contain a file extension - it will load up both the uexp and uasset files.

This function returns a Python representation (mostly Lists and Dictionaries) of the underlying UObject data. It will be a List of exports, each with the `export_type` parameter.

## Textures

The `read_texture(path: string, output_path: string)` function reads a file and writes it out into a texture.

Both of these functions accept a path of the same format as `read_asset`, and will throw an exception if the `export_type` of the first export is not a `Texture2D`

## Errors

At the moment, all errors in parsing/reading will return a TypeError with the error message matching that of what JohnWickParse would return.
