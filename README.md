# Minecraft-Face-Extractor
A simple tool to extract the face from a minecraft skin.

## Usage 
Usage: mcex --in <INPUT> --out <OUTPUT> --overlay <LAYER>

## Options
Options:

 -i, --in <INPUT>

 -o, --out <OUTPUT>

 -l, --overlay <LAYER>

Use -l yes to include the skin's overlay.

Use -l no to exclude the skin's overlay.

## Example
mcex -i skin.png -o skin_face.png -l yes

This command extracts the face from skin.png with the overlay and saves it as skin_face.png.
