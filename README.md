# gwep_chem_finder
A chemical recipe finder for the SS13 goonstation servers

## Usage
**This program is currently fairly early in development and currently lacks most of the planned features!**

The program updates the Chemistry-Recipes.dm script from the Goonstation repo, parses it into a Vec of Rust Structs, and then serializes this into a local .json file to be deserialized for use in the code. This Vec of structs is then converted into a tree that contains all of the required reagents. The main functionality that can be used currently is the `.print_dispenser_format()` of a compound tree. This is the base of what the program aims to be but the core is currently being refined to remove some bugs from parsing another script with all the random edge cases before moving forward with more intricate functionality. Printing all the trees can be a nice way to easily see all the recipes in the game minus alternative recipes (currently being fixed).

## Goal of the program

This program aims to eventually provide a GUI experience to make creating perfect beakers simple by creating optimized chemical recipes and presenting them to the user in copy/pastable forms where possible and listing all other non-base reagents. 
