# clido
A tool for creating and maintaining a todo-list on the command line

## Table of Contents
 - [Introduction](#introduction)
 - [Examples](#examples)
 - [Installation and Usage](#installation-and-usage)
 - [Environment Variables](#environment-variables)
 - [ToDo](#todo)
 - [Acknowledgements](#acknowledgements)
 
## Introduction
After wasting tons of sticky notes in an attempt to keep myself organized, I decided I would benefit from a CLI
as I do spend a lot of my time on my computer / in the terminal.

I decided to write a Rust alternative to the well known [Task Warrior](https://taskwarrior.org/news/).
  
## Examples
```
 clido add "Write more examples for people" # Adds with no priority

 clido add -p low "Do more things"          # Adds a new todo with low priority

 clido list                                 # Outputs a table will all of the todos

 clido mark 0                               # Marks the 0th task as complete

 clido del 0                                # Deletes the 0th task
```

## Installation and Usage

### Installation
Currently, the only way to install clido is using cargo:
 - `cargo install clido` 

### Usage

Clido's commands follow the following format:

`clido [Subcommand] [Flags] [Options] [Input]`

clido has the following functionalities:

| Goal                        | Command    | Options                                                | Input                                                             |
|-----------------------------|------------|--------------------------------------------------------|-------------------------------------------------------------------|
| Add an item                 | clido add  | -s  --start<br>-d  --due<br>-p  --priority<br>-t --tags| "Put the task you want here"                                      |
| Delete an item              | clido del  |                                                        | The ID number of the task you<br>want to delete.                  |
| Mark an item as<br>complete | clido mark |                                                        | The ID number of the task you<br>want to mark complete.           |
| List items                  | clido list | -f --filter<br>-c --complete<br>-p --pending           | Filters that an item must have<br>to be shown                     |  

### Formatting Rules

Dates (--start / --due) can be input in the following ways:
 * DD-MM-YYYY
 * Long name of day (e.g. Monday, Tuesday, ... )
 * Abbreviation of the day (e.g. Mon, Tue, ...)

Tags (--tags) *must* be separated by a comma if using multiple tags. Examples:
 * `clido add -t school,math,homework "Page 45. Logarithms 10-23"`
 * `clido add -t home,chores,dishes "Run the dishwasher"` 
 * `clido add -t no-tag`

## Environment Variables
- `_CLIDO_DIR`
  - Specifies the directory in which clido should store its database.
  - The default value varies across OSes:
    
    | OS          | Path                                     | Example                                    |
    | ----------- | ---------------------------------------- | ------------------------------------------ |
    | Linux / BSD | `$XDG_DATA_HOME` or `$HOME/.local/share` | `/home/alice/.local/share`                 |
    | macOS       | `$HOME/Library/Application Support`      | `/Users/Alice/Library/Application Support` |
    | Windows     | `{FOLDERID_RoamingAppData}`              | `C:\Users\Alice\AppData\Roaming`           |

## ToDo
Clido is still lacking many of the features I would like, such as:
- [x] Groups (tags)
- [x] Filters
- [x] Named Dates
- [x] End Dates (via date)
- [ ] Recurring Tasks (Marks self as pending every specified interval)
- [x] Colors
- [ ] Color Themes
- [ ] Interactive marking / deleting using fzf
- [ ] Configurable output

I plan to be adding these in as I go, although I can't promise any dates. Feel free to leave suggestions
  for any other features that you would like to see on the list.
  
## Acknowledgements
A good portion of the code that I use was possible thanks to a GitHub user [ajeetdsouza](https://github.com/ajeetdsouza),
and their work on [Zoxide](https://github.com/ajeetdsouza/zoxide). Clido is using many of the same features to Zoxide in 
order to make things work efficiently and quickly. I found their use of temp files worthwhile, and wanted to make sure that
it would be similarly implemented for my program as well.
