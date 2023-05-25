#! /bin/python3
import argparse
import os

def add_32_padding(output_file, unaligned_len):
    missing_bytes = (32 - unaligned_len % 32) % 32
    filler = 0x0
    output_file.write(filler.to_bytes(missing_bytes))

# set input arguments
parser = argparse.ArgumentParser(description='A quick shell script to create an executable bin for oxos')
parser.add_argument('base')
parser.add_argument('binaries', nargs='+')
parser.add_argument('output')

args = parser.parse_args()

# init output file
base_filename = args.base
base = open(base_filename, 'rb').read()

output_file = open(args.output, 'wb')
output_file.write(base)

unaligned_len = len(base)
align_byte_count = 32
binary_filenames = args.binaries

# add padding till 0x83000
if unaligned_len < 0x3000:
    buffer_len = ((0x3000 - 32 * 2) - unaligned_len)
    output_file.write(b'\x00' * buffer_len)

for filename in binary_filenames:
    # add tlv to file
    input_content = open(filename, 'rb').read()
    length = len(input_content)
    typeval = 0x42
    typeval = typeval.to_bytes(align_byte_count)
    output_file.write(typeval)
    output_file.write(length.to_bytes(align_byte_count))

    output_file.write(input_content)
    unaligned_len = length

    # 32 bit align previous file
    add_32_padding(output_file, unaligned_len)

