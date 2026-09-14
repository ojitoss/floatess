# Contributing to **Floatess**

Thank you for your interest in contributing to this project!  

Please take a moment to read this document before you get started.

## How to sugget a feature
- **The goal**: What problem does feature solve?
- **The solution**: Detailed description of how your feature resolve this problem.

## How to report a bug
- **Clear title**: What problem the feature solve?
- **Steps to reproduce**: Methods used or operated themselves.
- **Expected**: What should be happend instead than happend.

## Contribute guide

### Prerequisite steps 
1. Fork this repository un your own account.
2. Clone in your local machine:
```bash
git clone https://github.com/[your-username]/floatess
```
3. Create a branch for your changes:
```bash
git checkout -b feature/[name]
```

### Code quiality
- Add or update tests for your changes (preferibly the order of commits was: test -> change (bassicaly TDD))

### Sumbiting a Pull Request
1. Execute in your terminal the next commands:
```bash
cargo test
cargo fmt
```
If the tests pass correclty, can make the next step.
2. Push your branch to you own fork:
```bash
git push -u origin feature/[name]
```
3. Open a Pull Request in the **main** branch.
4. Link your Pull Request to releated issue.
5. Wait the manteiner review and in the best case scenario, accept Pull Request, if not, i give you a feedback of why was rejected.

### Recommend Code Style
List of recommended ways to code in this project. This was only a recommendation, not a neccesary thing to the PR was accepted.

- Use 'loop' statment instead of 'while' statment, this because the break condition change their place, exmaple:
```rs
// With 'while' statment
while i > 0 {
    i -= 1;
}

// With 'loop' statment (recommended)
loop {
   if i <= 0 { break; }

   i -= 1;
}
```

### Commits structure 
This repository is based in *Conventional Commits*, but about this specific repo, also had many *scopes* based in depth (like *feat(x/y/z)*), in this depth had the next ones (all of them target about only one file, the name of the scope (with '.md' if the scope had no extension):
- [README](./README.md): Used to doucment why is this project as a general description.

- [CONTRIBUTING](./CONTRIBUTING.md): Used to indicate the guide of contributing.

- [Cargo.toml](./Cargo.toml): Used to change config of the workspaces.

Also had a the next sub contributing guide for a specific module for the workspace:

- [core](./core/CONTRIBUTING.md)
- [decimal](./decimal/CONTRIBUTING.md)

All of this had the next considerations:
- The sub modules about a specific root file than target this file (README, CONTRIBUTING and Cargo.toml), target same file but in they folder, example: 'some/README' target 'some/README.md' file.
- Target a the folder with the same name of the scope, example 'some' scope target de 'scope/' folder.