# Lyre

Lyre is a CLI tool for creating music with [Lindenmayer systems](https://en.wikipedia.org/wiki/L-system) (L-systems). It generates a MIDI file based on an L-system definition.

If you're not familiar with L-systems, feel free to read the [primer](#l-system-primer) below.

## Usage

Use the --help option for the full manual.

```
lyre [OPTIONS] -d <DEPTH> -o <OUT> <FILE>
```

For example,

```
lyre -d 5 -o example.mid system.txt
```

uses `system.txt` as the input file containing the L-system definition, performs 5 iterations on it, and outputs the MIDI file `example.mid`.

### Syntax

The following describes how to write a valid L-system file:

- A symbol can be any uppercase letter, or any of the following special characters: `[`, `]`, `+`, `-`.

- The syntax for a rule is `A => B` where `A` is a single symbol and `B` is a sequence of symbols. For example `S => SS` and `X => S+[X]-X` are both valid rules.

- A valid input file is a text file containing a list of rules, _each on a separate line_, followed by a sequence of symbols defining the axiom. The order of the rules does _not_ affect the final result, but the axiom must always come after the list of rules.

- Anything written after a `#` character is considered a comment and is ignored.

For example, the following is a valid L-system definition:

```
S => SS
X => S+[X]-X

X
```

In this example, `X` is the axiom.

## Making music

To generate music, the resulting string from the L-system is read from left to right. The characters `S`, `[`, `]`, `+`, `-` are special symbols which perform the following actions:

_Note: To keep things simple, all examples here start from the note C and use the chromatic scale_.

- `S`: Play a sixteenth note. Multiple consecutive `S`s are played as a single note, with the length of the note matching the number of `S`s. For example, `SS` will play a single note with the length of two sixteenth notes, a.k.a an eighth note, and `SSSS` will play a quarter note.
- `+`: Move the note to be played _up_ by a step defined by the scale. For example, `S+S` will play C and then C#.
- `-`: Move the note to be played _down_ by a step defined by the scale. For example, `S-S` will play C and then B.
- `[`: Push the current state into the stack. The state consists of simply the note to be played.
- `]`: Pop the state. For example, `S[+S]S` will play C, then C#, and finally C again.

## L-system primer

_Note: There are different [variations of L-systems](https://en.wikipedia.org/wiki/L-system#Variations). This section describes the one used in this program_.

L-systems are used to generate strings of symbols iteratively, and can be defined as a set of **symbols**, a set of **rules**, and an **axiom**, which is the initial string of symbols.

Each rule has a **predecessor** and a **successor**. The predecessor is a single symbol, and the successor is a string of symbols.

You apply the rules starting from the axiom. Whenever a symbol in the current string matches a rule's predecessor, it is replaced by the corresponding successor. A new string is formed after all the symbols have been replaced, and the process can start over. Any symbols that do _not_ match a rule's predecessor remain in place.

For example, consider the following L-system:

```
A => ABC # rule 1: 'A' turns into 'ABC'
B => A   # rule 2

A        # axiom
```

Starting from the axiom `A`, applying the rules once we get the string `ABC`. Applying the rules further we get `ABCAC`, then `ABCACABCC`, and so on.
