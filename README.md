<!-- This file conforms to the Standard Readme Style -->

# Make Your Reads

<!-- INSERT BANNER HERE -->

<!-- INSERT BADGES HERE -->
[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

<!-- INSERT SHORT DESCRIPTION HERE -->

Easily build common file of your repository, e.g. README, LICENSE, CODE_OF_CONDUCT, CONTRIBUTING and CHANGELOG.

<!-- LONG DESCRIPTION HERE -->

This project comes from the desire of having a common structure between my repos and also the undesire of copy-pasting files around. Thus _Make Your Reads_ is born, aiming to provide templates and basis for common files in coding projects, like a README and a License, following the best (very opiniated) specifications and conventions. You will find a simple CLI built in Rust (~~btw~~) using template files and a few parameters to get you started on your new project.

<!-- ## Table of Contents -->

<!-- ## Security -->
 ## Background 
 
This project is in fact a new version of a [old one](https://github.com/caio-bernardo/MakeReadme) I've abbandoned years ago since working on it didn't bring me anymore joy. Anyway, I commonly find begginners (including myself) struggling in structuring good repositories, from choosing a license to writing a CONTRIBUITING page, it can be difficult and I feel this things are not normally presented to us. So this projects aims to provide new developers with a good starting point, from where they can improve their work. You can quickly create a your files and fill with some basic information and improve them as your project evolves.

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

### F.A.Q.

#### How to choose a License?

First of all I recommend checking this amazing website which will help you with that, [Choose a License](https://choosealicense.com/).

Second of all, if your are working with code choose _MIT_ or _GNU Public License_ (see website above for differences, if uncertantity, choose MIT), else choose a _Creative Commons_.

#### How to make a good README?

See [README Standard](https://github.com/RichardLitt/standard-readme).

#### How to keep a Changelog?

See [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

### How to make a good CODE_OF_CONDUCT?

See [Contributor Convenat](https://www.contributor-covenant.org/)
 
### How to make a good CONTRIBUTING?

See [CONTRIBUTING.md](https://contributing.md/example/) (no its not the file in the repo)

## Maintainers 

[@Caio Bernardo](https://github.com/caio-bernardo)

 ## Acknowledgements 
 
 This project was draws inspiration from:

 - [readme-md-generator](https://github.com/kefranabg/readme-md-generator)
 - [Best-README-Template](https://github.com/othneildrew/Best-README-Template)
 - [ReadME-Generator](https://github.com/ShaanCoding/ReadME-Generator)
 
 The templates and licenses where choose from:

 - [Choose a License](https://choosealicense.com/)
 - [README Standard](https://github.com/RichardLitt/standard-readme)
 - [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
 - [Contributor Convenat](https://www.contributor-covenant.org/)
 - [CONTRIBUTING.md](https://contributing.md/example/)
 

## Contributing

Feel free to [Open a New Issue](/issue/new) or [Submit a Pull Request](/pull/new).
See our [CONTRIBUTING](/blob/main/CONTRIBUTING.md) file for more information about contribution in specific ways. 
Don't forget to check our [Code of Conduct](/blob/main/CODE_OF_CONDUCT.md) for the repository guidelines.


## License

This project is under the GPL-3.0-or-later license. For more info see [LICENSE](/blob/main/LICENSE).

Made with [Make Your Reads](https://github.com/caio-bernardo/make-your-reads).
