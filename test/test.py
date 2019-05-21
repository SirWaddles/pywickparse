import pywickparse

extractor = pywick.PakExtractor("pakchunk.pak", "0000000000000000000000000000000000000000000000000000000000000000")
files = extractor.get_file_list()
file = extractor.get_file(0)
print(files[0])
handle = open("./test", "wb")
handle.write(file)
handle.close()
