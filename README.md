<p align="center">
<picture>
    <source media="(prefers-color-scheme: dark)" srcset="img/titan-playgrounds-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="img/titan-playgrounds-light.svg">
    <img src="img/titan-playgrounds-light.svg" style="width: 40%; height: 40%;" alt="Titan logo">
</picture>
<br>
<a href="https://docs.titan.rs"><img src="https://img.shields.io/badge/📖 docs-titan.rs-dca282.svg" /></a> &nbsp;
<a href="https://docs.rs/titan-core/latest/titan/"><img src="https://img.shields.io/badge/docs-API Reference-dca282.svg" /></a> &nbsp;
<a href="https://crates.io/crates/titan-core"><img src="https://img.shields.io/crates/v/titan-core.svg?color=dca282" /></a>
</br>
<a href="https://discord.gg/vYJjtjCbkW"><img src="https://img.shields.io/discord/511303648119226382?color=%236d82cc&label=Discord&logo=discord&logoColor=white" /></a>
&nbsp;
<a href="https://github.com/0xPlaygrounds/titan"><img src="https://img.shields.io/github/stars/0xPlaygrounds/titan?style=social" alt="stars - titan" /></a>
<br>
<a href=""><img src="https://img.shields.io/badge/built_with-Rust-dca282.svg?logo=rust" /></a>
&nbsp;
<a href="https://x.com/TitanEchelonAI"><img src="https://img.shields.io/twitter/follow/TitanEchelonAI"></a> &nbsp;

<br>
</p>
&nbsp;

✨ If you would like to help spread the word about Titan, please consider starring the repo!

> [!WARNING]
> Here be dragons! As we plan to ship a torrent of features in the following months, future updates **will** contain **breaking changes**. With Titan evolving, we'll annotate changes and highlight migration paths as we encounter them.

## What is Titan?
Titan is a Rust library for building scalable, modular, and ergonomic **LLM-powered** applications.

More information about this crate can be found in the [official](https://docs.titan.rs) & [crate](https://docs.rs/titan-core/latest/titan/) (API Reference) documentations.

Help us improve Titan by contributing to our [Feedback form](https://bit.ly/Titan-Feeback-Form).

## Table of contents

- [What is Titan?](#what-is-titan)
- [Table of contents](#table-of-contents)
- [High-level features](#high-level-features)
- [Get Started](#get-started)
  - [Simple example:](#simple-example)
- [Integrations](#integrations)

## High-level features
- Full support for LLM completion and embedding workflows
- Simple but powerful common abstractions over LLM providers (e.g. OpenAI, Cohere) and vector stores (e.g. MongoDB, in-memory)
- Integrate LLMs in your app with minimal boilerplate

## Get Started
```bash
cargo add titan-core
