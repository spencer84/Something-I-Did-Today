# Something I Did Today
Something I did today (sidt) is a command-line application designed for a minimalist journaling experience. The idea of the app is to allow users to quickly write about what they have done in a given day. The app will automatically append each entry to a text file with the current date, so all that is needed from the user is a sentance or two about their day. 

## Installation
Run ```Cargo build``` from within the directory to build the binary file.

## Usage
Writing an entry requires no arguments by default. The application will take the full input as an entry and format the current date.
For example:
```sidt Today I went for a walk.``` Will write the following line to the journal.txt file:
27/5/24 Today I went for a walk.

The following arguments are also supported:

- "l" : Read last entry
- "r <x>" : Read the last x number of lines
- "y" : Write an entry for yesterday's date


