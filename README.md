# Fade
Fade is a command-line utility used for create fade animated gif.
# Example
![Example](https://raw.githubusercontent.com/oupson/Fade/master/images/map.gif)

Here, the file have been compressed and his size reduced.
Generally, it produce bigger gif.
# Usage
```
fade <file 1> <file 2> [options]
Options :
    -o <output path> Set output path.
    -w Write frames to disk.
    -a Write a .json used by apngasm.
    -n <count> Set frames count.
    -d <important> <standard> Set durations of frame in ms
    -s <speed> Set gif conversion speed. Must be between 1 and 30, 30 is loss quality but faster.
    -r <width> <height> Resize image.
    
Examples :
    fade image1.jpg image2.jpg will create an animation from the 2 images
    fade *.png -o o.gif -n 50 will take every images in the directory that end with .png, output the result to o.gif and with 50 frames per images
```

# Install
```
cargo install --git https://github.com/oupson/Fade
```
