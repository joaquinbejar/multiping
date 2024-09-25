# MultiPing: Rust command-line utility for pinging multiple hosts

[![Dual License](https://img.shields.io/badge/license-MIT%20and%20Apache%202.0-blue)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/multiping.svg)](https://crates.io/crates/multiping)
[![Downloads](https://img.shields.io/crates/d/multiping.svg)](https://crates.io/crates/multiping)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/multiping.svg)](https://github.com/joaquinbejar/multiping/stargazers)

[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/multiping/CI)](https://github.com/joaquinbejar/multiping/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/multiping)](https://codecov.io/gh/joaquinbejar/multiping)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/multiping)](https://libraries.io/github/joaquinbejar/multiping)

## Table of Contents
1. [Introduction](#introduction)
2. [Features](#features)
3. [Project Structure](#project-structure)
4. [Setup Instructions](#setup-instructions)
5. [Library Usage](#library-usage)
6. [Usage Examples](#usage-examples)
7. [Development](#development)
8. [Contribution and Contact](#contribution-and-contact)

## Introduction

multiping is a Rust-based concurrent tool designed to monitor multiple IP addresses simultaneously by sending ICMP ping requests and gathering real-time statistics for each target. It leverages multi-threading to ensure efficient performance, allowing each IP to be pinged independently. Key features include:

Customizable ping behavior: Sends a configurable number of ICMP ping requests (default is 3) with a user-defined interval between each ping (e.g., -c 3 -i 0.2 for three pings with 0.2 seconds between them).
Real-time statistics: Tracks and displays the total number of attempts, successes, and failures for each IP address being monitored.
Concurrency: Uses Rust's threading model to handle multiple IPs simultaneously without blocking, ensuring a fast and scalable solution.
Configurable via command line: Pass any number of IPs as input parameters to monitor, making it flexible for network monitoring scenarios.

## Usage

```shell
multiping <IP_1> <IP_2> ... <IP_N>
```

```shell
multiping 8.8.8.8 1.1.1.1 192.168.1.1
```

## Features

- Efficient multi-threaded pinging for each IP.
- Tracks attempts, successes, and failures.
- Real-time display of network statistics.
- Can monitor an arbitrary number of IP addresses concurrently.

## Development

This project includes a Makefile for common development tasks:

- `make build`: Build the project
- `make test`: Run tests
- `make fmt`: Format the code
- `make lint`: Run Clippy for linting
- `make clean`: Clean the project
- `make check`: Run pre-push checks (test, format check, lint)
- `make coverage`: Generate test coverage report
- `make doc`: Generate and open documentation

To run a specific task, use `make <task_name>`. For example:

```shell
make test
```

## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

**Joaquín Béjar García**
- Email: jb@taunais.com
- GitHub: [joaquinbejar](https://github.com/joaquinbejar)

We appreciate your interest and look forward to your contributions !
