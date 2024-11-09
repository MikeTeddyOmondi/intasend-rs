# Intasend - Documentation

## Prerequisites

Install mdbook using `cargo` - Rust's package manager

```shell
cargo install mdbook
```

---

### Serve the docs

Launch the docs

```shell
mdbook serve
```

---

### Build the docs to static assets

This command builds the docs and export assets (HTML, CSS & JavaScript) to the `book` directory

```shell
mdbook build
```

---

### Do you have Docker installed? 

##### Using Docker 

Package the docs site with `Docker`

```shell
docker build -t intasend-docs . 
```

Launch a `Docker` container using the image built previously

```shell
docker run -d -p 3000:3000 --name docs intasend-docs
```

---
