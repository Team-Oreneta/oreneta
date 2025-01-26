#!/usr/bin/env python
from PIL import Image
from sys import argv

VERSION_MAJOR = 0
VERSION_MINOR = 1

def image_to_oiff(image_path, output_path):
    img = Image.open(image_path)

    with open(output_path, "wb") as f:
        # Magic number; Oreneta Image File Format
        f.write(b"OIFF")

        # Version number major
        f.write(VERSION_MAJOR.to_bytes(2, byteorder="little"))

        # Version number minor
        f.write(VERSION_MINOR.to_bytes(2, byteorder="little"))

        # Image width
        f.write(img.width.to_bytes(4, byteorder="little"))
        
        # Image height
        f.write(img.height.to_bytes(4, byteorder="little"))

        # Pad with zeroes
        padding_needed = 128 - f.tell()
        if padding_needed > 0:
            # pass
            f.write(b'\0' * padding_needed)

        for y in range(img.height):
            for x in range(img.width):
                r, g, b, a = img.getpixel((x, y))
                pixel = bytes(bytearray([b, g, r, a]))
                f.write(pixel)

def main():
    if len(argv) < 3:
        print("Usage: png_to_oiff.py <input_image> <output_image>")
    else:
        image_to_oiff(argv[1], argv[2])

if __name__ == "__main__":
    main()