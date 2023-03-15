# rustpal: A ChatGPT assistant built with Rust

I'm using this to learn Rust. It was fun to build.

https://user-images.githubusercontent.com/848147/225391449-694f90ba-7397-4a31-9e60-437e44c3ed09.mp4

## Usage

Install dependencies:

```bash
cargo install
```

Get your [OpenAI API key](https://platform.openai.com/account/api-keys) and export it as an environment variable in your shell:

```bash
export OPENAI_API_KEY=sk-...
```

Then, run the program:

```bash
cargo run -- "Hello. How tall is the Empire State Building?"
```

Or build it:

```bash
cargo build
```

And run it:

```bash
./target/debug/rustpal "Hello. How tall is the Empire State Building?"
```
