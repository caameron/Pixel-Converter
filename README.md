# Pixel-Converter
Pixel Converter for Rust Programming Language Class

## How to run program
Download the repository and have the path of the image you want to alter on hand

Switch into the src directory.
(This is still a work in progress, so for now the program has to run in the src directory or else the program will not be able
to save the output image correctly)

```
run : cargo run
```

All crates will be downloaded and the program will prompt you to give the path of the image you want to alter.
The program will then display a menu and give you instructions

When finished the ouput of the program will be saved to the Output directory which is located with the source files in
the src directory

## TESTING
To test the program a simple command will run the tests

```
cargo test
```

this will run a series of unit tests that test individual functions. The tests will run the function on an image and compare each pixel
with the pixels of a image which has already been altered in the same way and confirmed that it is correct.

# Topic Area and Project Vision
For this project we want to focus on image manipulation by changing its pixels. So our topic area revolves around images and graphics and
the unique and vast ways they can be changed or altered to match someones preferences or creativity.

Our Vision for this project is to create an interface where the user will be able to supply our product with an image and decide what
altercations they want done on it. The alterations will range from size, color, orientation, blurriness, etc. We want the user to be
able to have full control over what they want to happen to their image. Also we want to add in features which will be interesting for us
as well. By that we mean, using the rust tools which we have learned to implement a feature which may not already have been thought of.
All of this will be command line based, unless we have enough time to implement a sofphisticated GUI.

# Layout of Project
There will be multiple files that will run the program. The main.rs file will take in an image from the command line and output a command-line menu
where the user will be able to pick and choose what alterations they want to have done to the image. Each of the alterations will be implemented
in different rust files and have functions that can be called from the main file. The output of the program will be another image with all of the
alterations done to it.

To implement this project we will first familiarize ourselves with the image crate. This will allow us to have access to the individual
pixels in the image as well as making changes to them (We will not be using the built in functions which can alter the images
we plan on implementing any functions that alter pixels ourselves). Next we will make our program able to take in and output an image.
These first two tasks will be done fairly quickly in about a week. In the next phase we will begin implementating features for our program
which will alter the pixels of the image passed in. This phase is fairly broad because we haven't decided on the full list of features
to implement yet. But this will be the bulk of the project and take until the last week of class.

# Issues and Concerns
The main issue/concern about the project is how much control we will have over the pixels from the image crate. We don't know much information
about this crate and it will be a big learning curve. However, other than that there shouldn't be too many issues besides the normal predicatable
ones. For example, bugs or problems with the borrow checker which we should be able to solve in an orderly manner.

# What was built
The program that we created matched what our project vision stated. We were able to create a command line program which took in a path to an image as
input and altered that image based on the users choices of pre-defined features. The features uses the image crate to obtain each individual pixel of
the image and make alteractions to them in order to create the end product. Any type of image file is acceptable as long as the path given to the
program is valid but the file given back is a PNG. Overall it is a program which alters the pixels of the image to create a brand new image which is
exactly what we wanted to create.

# How it works?
The program first obtains the path to the image file from the user and uses the image crate to open that file which gives back a dynamic image. From
there a series of function can be called on that image to give back certain traits such as the height and width of the picture and an iterator over
all its pixels. We created a struct which held that image and its dimensions which was then passed around to different functions which would call
the iterator over the pixels and alter them in different ways. All the functions would return the struct back with the new image in the struct, and
from there it could be passed in to more functions to be altered even more. Once finished the program grabs the dynamic image converts it to a png
file and places it in a directory for the user to obtain.

# What doesn't work?
We tried to have as little non-working functions in the final build as possible however their are many features which we did not get to implement
which we wanted to have. This means that the project works fine and all the features work as intended but, there are still many features which 
would make sense to have that are not there yet. For example we wanted an easier way to give the output image back instead of just placing it in
a directory along the source files or implementing a better way for the user to give us their image instead of manually typing in the path of the
image which is very tedious and prone to errors.

# Lessons Learned
The project was a very valuable learning experience and a lot of was learned about processing images in Rust. Especially working with pixels and
obtaining the different attributes of images using the image crate. Besides that the project let us learn a lot about using crates in Rust and
implementing multiple files. It also solidified the foundation for the Rust basics in us. Concepts such as iterators, structs, and working with
the borrow checker are much clearer and we feel completely comfortable working with them now after the project.

