# gitlab-ci-validate-rust

Checks if your .gitlab-ci.yml file is valid.

> I decided to create a simple cli tool to try Rust so this repository is a fork of my nodejs version https://github.com/pradel/gitlab-ci-validate.

## Install

```
$ cargo install gitlab-ci-validate
```

## Usage

```sh
# To see all the available options
$ gitlab-ci-validate --help

# It will try to find if .gitlab-ci.yml is present on the current folder if no file path is provided
$ gitlab-ci-validate <file-path>

# You can specify your own gitlab host if you need
$ gitlab-ci-validate <file-path> --host https://my-gitlab-url

# You can provide a private token if you need
$ gitlab-ci-validate <file-path> --private-token <my-gitlab-token>
```

## License

MIT © [Léo Pradel](https://www.leopradel.com/)
