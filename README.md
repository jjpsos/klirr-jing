# Installation

## Font

```sh
brew tap homebrew/cask-fonts
brew install --cask font-computer-modern
```

# Run

Clone and run:

```sh
make
```

or:

```sh
make help
```

## Invoice for services

```sh
cargo run --bin invoice
```

or for current month instead of last:
```sh
cargo run --bin invoice -- --month current
```

or if you want to configure output:
```sh
cargo run --bin invoice -- --output
```

### Out of office for some days?

```sh
cargo run --bin invoice -- ooo 5
```

## Invoice for expenses



```sh
cargo run --bin invoice -- expenses "Coffee,4.25,GBP,2,2025-05-23" "Sandwich,8.75,EUR,1,2025-05-29"
```

or if you wanna open the PDF after successful build:

# Development

## Precommit

Install [`pre-commit` tool](https://pre-commit.com)

```sh
brew install pre-commit
```

## Imagemagick

Install [`imagemagick`](https://imagemagick.org)

```sh
brew install imagemagick
```
