# gwep_chem_finder
A chemical recipe finder for the SS13 goonstation servers

## Usage
**This program is currently fairly early in development and currently lacks most of the planned features!**

The program updates the Chemistry-Recipes.dm script from the Goonstation repo, parses it into a Vec of Rust Structs, and then serializes this into a local .json file to be deserialized for use in the code. This Vec of structs is then converted into a tree that contains all of the required reagents. The program can be run in CLI mode now with -c or --cli as a launch argument ex. `cargo run -- -c`.

The program takes a reaction to display and also commands as input with "/".

Use `/help` or `/h` to see all commands.

## Goal of the program

This program aims to eventually provide a GUI experience to make creating perfect beakers simple by creating optimized chemical recipes.
