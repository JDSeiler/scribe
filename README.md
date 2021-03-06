# Scribe
Scribe is a command line utility for converting the binary data files of the MNIST data set (see: http://yann.lecun.com/exdb/mnist/)
to a collection of BMP images.
## Running the script
To compile, run `cargo build --release`. You will find the executable in the directory `./target/release`. 

Then, at the same level as the `src` directory, create the following directories:
```
./out/0
./out/1
./out/2
./out/3
./out/4
./out/5
./out/6
./out/7
./out/8
./out/9
```

The script requires 4 parameters:
- Two paths relative to the current working directory
  - The location of the image data file
  - The location of the corresponding labels file
- A 1-based index describing which image to start with
- How many images to read

For example, if your directory structure looks like this:
```
./mnist
  t10k-images-idx3-ubyte
  t10k-labels-idx1-ubyte
  train-images-idx3-ubyte
  train-labels-idx1-ubyte
```
You could invoke the script like this to dump all 10,000 testing images:
`./target/release/scribe ./mnist/t10k-images-idx3-ubyte ./mnist/t10k-labels-idx1-ubyte 1 10000`

As another example, you could just dump 3,000 images, starting from image number 6,000:
`./target/release/scribe ./mnist/t10k-images-idx3-ubyte ./mnist/t10k-labels-idx1-ubyte 6000 3000`

The script will output images in a directory called `out`. Images are organized by the type into
separate folders. That is, all the zeros go into `./out/0`, all the ones into `./out/1`, etc.
Each image is named in the following pattern: `d{type}-{id}.bmp` so the name: `d5-0040.bmp` would
indicate this image is a 5 (denoted by d5) and it is the 41st 5 out of all 5s read from the dataset.
The images are 0 indexed, which is why `0040` is the 41st image.

You can also print a help message by invoking the script with a single option: `--help`, like so:
`scribe --help`.

The tool does not have very robust error reporting and is not configurable aside from the options
described here, but it is functional!
