# Architecture For the Redis Protocol Server

## Introduction

The goal of this project is two folds:

- Learn the Redis protocol and how to implement a server that can communicate with Redis clients.
- Learn how to do network programming in Rust

## Requirements

- The server should be able to handle multiple clients concurrently.
- The server should be fully compliant with the Redis protocol, and compatible with any existing Redis clients (e.g. redis-cli).

## Architecture

#### Handling multiple clients concurrently

- An event loop should be implemented to handle multiple clients concurrently, using the poll API.

## TODO

- [ ] Understand the existing socket server implementation in C++.

# Questions
