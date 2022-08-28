# ezsh

A simple and easy to read shell.

## Motivation

The motivation of this program is twofold.

The first reason I crated this is for teaching purposes, having an easy to read
implementation of a shell might help understand what a shell actually is.

The second reason is that this minimal (somewhat-)useful program could be
implemented in different programming languages for an easy comparison
of different languages and how you would solve a simple problem like a shell
program in them.

## Features

`ezsh` is not intended for productive usage. Its feature set is intentionally
very limited and the code is written primarily for easy readability, not
for robustness, performance or other usually desirable properties.
The features are:

* built-in commands `echo`, `cd` and `exit`
* basic prompt
* launch external programs
* argument splitting and quotes

There are known limitations such as the inability to pass quotes to a program
due to the lack of escapes.

Features that are not currently available but might be considered:

* setting env variables
* redirecting IO

## License

The files provided in this repository are licensed at either CC0 or MIT (Expat) at your option.

