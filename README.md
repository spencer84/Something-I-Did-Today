# Something I Did Today
Something I did today (sidt) is a command-line application designed for a minimalist journaling experience. The idea of the app is to allow users to quickly write about what they have done in a given day. The app will automatically append each entry to a text file with the current date, so all that is needed from the user is a sentance or two about their day. 

## Installation
Run ```cargo install --path .``` from within the directory to build the binary file.

## Usage
Writing an entry requires no arguments by default. The application will take the full input as an entry and format the current date.
For example:
```sidt Today I went for a walk.``` Will write the following line to the journal.txt file:
> 2024-05-24 Today I went for a walk.

The following arguments are also supported:

### -l,  --last  
Read last entry   
Usage: ```sidt -l```
> 2024-05-24 Today I went for a walk.

### -r <number>,  --read <number>  
Read a specified number of previous entries. Defaults to last 5 entries.   
Usage: ```sidt -r 5```
> 2024-05-24 Today I went for a walk.  
> 2024-05-23 Went for a coffee in the morning.  
> 2024-05-22 Did some work around the house.  
> 2024-05-21 Dinner out at the cool new restaurant in town.  
> 2024-05-20 Slow day at work.   

Alternatively, all records can be viewed using the *all* argument (or simply *a*):
```sidt -r all``` or ```sidt -r a```

  
### -y,  --yesterday  
Read yesterday's entry.  
Usage: ```sidt -y```
> 2024-05-23 Went for a coffee in the morning.  

### -s,  --search  
Search entries for a phrase.  
Usage: ```sidt -s <phrase>```

### -e <date>, --edit <date>
Edit a previous entry.
Usage: ```sidt -e 23052024```
Terminal will pre-populate the previous entry:
> 2024-05-24 Today I went for a walk. <Cursor appears here to begin editing>

> 2024-05-24 Today I went for a walk to the park.

A new line will end the entry and update the entry in the database:
```sidt -r 1```
> 2024-05-24 Today I went for a walk to the park.


