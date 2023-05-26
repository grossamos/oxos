#! /bin/python3
import argparse
import os

def padd_until_prog(prog_num, bin_len, output_file):
    buffer_len = (0x3000 + 0x1000 * prog_num - 32 * 2) - bin_len
    output_file.write(b'\x00' * buffer_len)
    return buffer_len

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

bin_len = len(base)
align_byte_count = 32
binary_filenames = args.binaries

counter = 0
for filename in binary_filenames:
    bin_len += padd_until_prog(counter, bin_len, output_file)

    # add tlv to file
    input_content = open(filename, 'rb').read()
    length = len(input_content)
    typeval = 0x42
    typeval = typeval.to_bytes(align_byte_count)
    output_file.write(typeval)
    output_file.write(length.to_bytes(align_byte_count))

    output_file.write(input_content)
    print(length)
    bin_len += length + 32*2

    counter += 1


