# Nucleos

*A fast, safe, declarative system-management tool for Unix machines — powered by Rust and Lua.*


## What is Nucleos?

So far? Nothing. At this stage, Nucleos is just a set of ideas and there is very real possibility it will stay that way forever.
If I do still think that this is a good idea after getting started on some of the basics, maybe this will turn into something more useful.
We'll see.
Anyway, here's the concept so far:

## Overview

Nucleos is an experimental system-management and dotfile-orchestration tool designed to fill the gap between fully fledged configuration managers like Ansible and lightweight dotfile managers like GNU Stow.  
Unlike nix-based solutions, Nucleos is portable across Unix-like systems without requiring a separate package manager or language runtime.  
It focuses exclusively on local system management, providing a simple yet powerful way to describe the desired state of your machine.

The core idea:  
Define tasks, apply them, track their state, and automatically undo their effects when they are removed.

## Goals

- Declarative tasks with undo  
  Every task defines when it is applied, when it is up-to-date, and how to undo itself when removed.

- State tracking built in  
  Enables reversible, reproducible machine configuration.

- Rust core, Lua configuration  
  Rust for performance, safety, and extensibility.  
  Lua for ergonomic configuration and authoring custom modules.

- Modules similar to Ansible  
  Built-in modules (file, git, etc.), custom Lua modules, maybe extendable Rust-based modules.

- Great UX  
  Intuitive CLI, pretty formatted output, and inspectable logs.  
  Future ideas include parallel task execution and a TUI for interactive management.

- Zero-dependency bootstrapping  
  A hosted bootstrap endpoint capable of installing the tool and applying a user’s configuration in one go, for example:
  curl deploy.example.com/username | bash

## Status

This project is in the very early stages.  
There is no usable code yet, and everything described here is subject to change as the design evolves.

