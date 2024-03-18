# Tagcm

tagcm is a command line tool for tagging commands. It allows you to some functions to manage commands.

## Installation

WIP

## Usage

### Add a tag and command

To add a tag, use the `add` command followed by the tag name and the command.

```
tagcm add <tag> <command>
```

if you add the tag which is already exist, the command will be replaced.

### Delete a tag and command.

To delete a tag and command, use the `delete` command followed by the tag name.

```
tagcm delete <tag>
```

### Show tags

To show all tags and commands, use the `show` command with the target as "all".

```
tagcm show all
```

To show specific tag and command, you can specify the tag.

```
tagcm show <tag>
```

### Search tags

#### with search str.

To search tags and commands, use the `search` command with the tag prefix string.

```
tagcm search <search_str>
```

e.g.

```
tagcm add some_tag_1 "echo some_tag_1"
tagcm add some_tag_2 "echo some_tag_2"
tagcm add hoge_tag_1 "echo hoge_tag_1"

tagcm search some

->(Result)
tag: some_tag_2, command: echo some_tag_2
tag: some_tag_1, command: echo some_tag_1

```

#### interactive search(CUI search)

you can use search command without any search_str, then it runs interactive CUI search interface.

on the interface you can search

```
tagcm search
```

```
Press any key:to start auto-complete tag and command,.
key Left, key Right:move cursor in INPUT window.
key Up, key Down:move cursor in Search results window.
Enter:to choose the command to clipboard and exit search mode.
Esc:to exit search mode.
```

### Data Storage

The data is stored in a JSON file named `tags.json`. The path to this file can be specified using the --data-path option. If no path is specified, the file is stored in the default configuration directory.

### License

MIT
