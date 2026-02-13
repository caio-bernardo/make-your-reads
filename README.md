<!-- This file conforms to the Standard Readme Style -->

# Make Your Reads

<!-- INSERT BANNER HERE -->

<!-- INSERT BADGES HERE -->
[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

<!-- INSERT SHORT DESCRIPTION HERE -->

Easily build common file of your repository, e.g. README, LICENSE, CODE_OF_CONDUCT, CONTRIBUTING and CHANGELOG.

<!-- LONG DESCRIPTION HERE -->

This project comes from the desire of having a common structure between my repos and also the undesire of copy-pasting files around. Thus _Make Your Reads_ is born, aiming to provide templates and basis of common files in coding projects, like a README and a License, following the best (very opiniated) specifications and conventions. You will find a simple CLI built in Rust (~~btw~~) using template files and a few parameters to get you started on your new project.

<!-- ## Table of Contents -->

<!-- ## Security -->
<!-- ## Background -->

## Install

### Cargo

You can install using Cargo package manager.

```sh
cargo install make-your-reads
```

### Downloading

You can download the prebuilt binaries in the [release section](/releases)

### Building Manually

You can build and install from source (which requires Rust latest compiler).

```sh
cargo install --git https://github.com/your_username/make-your-reads.git
```

## Usage

**Basic usage**

```sh
mkyr readme <project-name>
```

**Choosing a License**

```sh
mkyr license mit
```

**Change output path**

```sh
mkyr readme <project_name> -p docs/Readme.md
```

### Options

```txt
Usage: mkyr [OPTIONS] [COMMAND]

Commands:
  readme     Creates a new README file
  license    Creates a new License file
  coc        Creates a new Code of Conduct file
  contrib    Creates a new Contributing file
  changelog  Creates a new Changelog file
  help       Print this message or the help of the given subcommand(s)

Options:
  -f, --force        overrides file with same output name
  -p, --path <PATH>  output file path
  -h, --help         Print help
  -V, --version      Print version
```

## Maintainers 

[@Caio Bernardo](https://github.com/caio-bernardo)

<!-- ## Acknowledgements -->

## Contributing

Feel free to [Open a New Issue](/issue/new) or [Submit a Pull Request](/pull/new).
See our [CONTRIBUTING](/blob/main/CONTRIBUTING.md) file for more information in how to contribute in more specific ways. 
Don't forget to check our [Code of Conduct](/blob/main/CODE_OF_CONDUCT.md) for the repository guidelines.


## License

This project is under the MIT license. For more info see [LICENSE](/blob/main/LICENSE).

Made with [Make Your Reads](https://github.com/caio-bernardo/make-your-reads).
