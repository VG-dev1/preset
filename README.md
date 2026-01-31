# preset

[![Version info](https://img.shields.io/crates/v/preset.svg)](https://crates.io/crates/preset)
![Crates.io Total Downloads](https://img.shields.io/crates/d/preset)
![Crates.io License](https://img.shields.io/crates/l/preset)

`preset` is a program for managing and running command sequences at once, so you don't have to manually type your commands - just append them to your preset and run your preset.

[Installation](#installation) • [Usage](#usage)

## Features
- Create, delete and manage your presets
- Run your presets
- Placeholders for flexible values (user input)
- JSON saving
- Debugging messages

## Demo

![Demo](assets/demo.gif)

## Usage

### Create a preset

Your first step is to create your preset. You can do that by running `preset create`, followed by your preset's name:

```bash
preset create "test"
```

### Append commands

After creating a preset, you need to add your commands to it. You can do that by running `preset append`, followed by your preset's name and a command:

```bash
preset append "test" "echo Hi!"
preset append "test" "echo This is a test!"
```

#### Placeholders

Normally, the appended commands's value is fixed (e.g. `echo Hi!`). You can make a value flexible (make a preset ask for input when it reaches that command) by adding a placeholder `{}`:

```bash
preset append "test" "echo {}"
```

### Run a preset

Finally, you can run your preset by running `preset run`, followed by your preset's name:

```bash
preset run "test"
```

You can also disable debugging messages using the `--no-message` flag.

> [!NOTE]
> If a command returns non-zero, the preset will crash. To keep the preset running despite a failed command, use the `--skip-errors` flag.

### Remove commands

You can remove a specific command from your preset by running `preset remove`, followed by your preset's name and a command:

```bash
preset remove "test" "echo Hi!"
```

### Pop commands at a given index

You can pop a command from your preset at a given index by running `preset pop`, followed by your preset's name and an index:

```bash
preset pop "test" 0
```

### Insert commands

You can insert a command at a given index by running `preset insert`, followed by your preset's name, index and command:

```bash
preset insert "test" 0 "echo Hi!"
```

### Listing presets

You can see all of your created presets by running `preset list`:
```bash
preset list
```

This prints the JSON where all of your created presets are saved.

### Installation

#### From source

```bash
cargo install preset
```

## Licence

This project is distributed under the MIT License.