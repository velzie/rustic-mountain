# partially stolen from https://github.com/weewStack/Python-projects/blob/master/000-BMP-Converter/n5110_BMP_converter.py
# will convert a given bitmap into an array usable in the atlas

import sys
import struct 



file_name = sys.argv[1]
file_name = file_name.split(".")[0]


with open(f'{file_name}.bmp', 'rb') as bmp:

    bmp.seek(10, 0)
    offset = struct.unpack('I', bmp.read(4))[0]

    bmp.seek(18, 0)
    bmp_w = struct.unpack('I', bmp.read(4))[0]
    bmp_h = struct.unpack('I', bmp.read(4))[0]


    bmp.seek(34, 0)
    bmp_s = struct.unpack('I', bmp.read(4))[0]

    bmp_b = int(bmp_s/bmp_h)

    bmp.seek(offset, 0)

    bmp_line = ''
    bmp_list = []
    bmp_list_v = []

    out = ""


    for line in range(bmp_h):
        for byte in range(bmp_b):
            bmp_byte = bmp.read(1)
            bits = format(255-struct.unpack('B', bmp_byte)[0], "08b")
            bmp_line += bits
        bmp_list.append(bmp_line[:bmp_w])
        bmp_list_v.append(bmp_line[:bmp_w])
        bmp_line = ''
    bmp_list_v.reverse()

    for line in bmp_list_v:
        out += line
    print(f"\"{out}\"")
