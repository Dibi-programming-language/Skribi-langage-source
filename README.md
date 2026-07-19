# Scribi-langage-source

WARNING: this version of Skribi is not usable as a rework is in progress.

Skribi is a programming language for writing code in the Dibi conlang.

Links :

- [Dibi dictionary](http://dibi-leksiro.fr/app/)
- [Dibistan wiki](https://dibistan.fandom.com/fr/wiki/Dibistan)
- [Skribi discord](https://discord.gg/eGYQVGd4k3)
- [Dibi discord](https://discord.gg/CKnqUxQaMq)

## Build and run from source

### Using nix

If you have nix, you do not have to install anything.

- `nix run` will run the code
- `nix build` with produce a binary in the `result` directory
- `nix develop -c [your shell]` will start a shell with everything installed,
  and run your shell inside.

### Using cargo

You can use ``cargo run -- <file to run>`` to run the project.

> [!warning]
> Please use this directory as the working directory, not `src`.

## Contributing

You are free to contribute if you want to. Please however join the discord
before: as this is a small and WIP project (for many years now), we are mainly
using discord to take design decisions.

## Thanks

### Tools

#### Mermaid

The debug output for the AST is in the Mermaid format :

> Sveidqvist, K., & Contributors to Mermaid. (2014). Mermaid: Generate diagrams from markdown-like
> text [Computer software]. https://github.com/mermaid-js/mermaid

