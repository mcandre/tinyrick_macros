# DEVELOPMENT GUIDE

tinyrick_macros follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 20.10.21+
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+
* Provision additional dev tools with `make -f install.mk`

## Recommended

* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)

# SECURITY AUDIT

```console
$ make audit
```

# BUILD: LINT, DOC, COMPILE, and TEST

```console
$ make build
```

# PUBLISH

```console
$ make publish
```

# CLEAN

```console
$ make clean
```
