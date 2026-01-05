# TreX

TreX (pronounced 'trek') is a make-like tool designed for multi-file markup
languages. 

It was created because of the difficulty in setting up make files for use with
lecture notes, where there is a common preamble, and a collection of notes
files, one of which was to be selected and used by `pdflatex`. While using
`make`, the result was a number of make variables which needed to be remembered,
and a command which was not super easy to remember. 

Perhaps the greatest downside in using `make` for this task was managing section
and subsection numbering. This had to be done manually for each lecture via
command line variables. 

TreX avoids this by simplifying the configuration setup, and providing short,
easy-to-remember commands for compilation, all while managing section and
subsection numbers for you. 

# Installation

It is recommended to install using cargo. 

- clone this project with
```
git clone https://github.com/bradencarlson/trex
```
- Enter the project
```
cd trex
```
- Build it
```
cargo build --release
```
- The binary can now be found in the `targets/release` directory. Place it
  somewhere in the `$PATH`.

# Documentation

Currently, there is no documentation for this project. I am working on it!
