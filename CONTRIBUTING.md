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
git clone https://ojitoss/floatess
```
3. Create a branch for your changes:
```bash
git checkout -b feature/[name]
```

### Code quiality
- Add or update tests for your changes (preferibly the order of commits was: change -> tests)

### Sumbiting a Pull Request
1. Push your branch to you own fork:
```bash
git push -u origin feature/[name]
```
2. Open a Pull Request in the **main** branch.
3. Link your Pull Request to releated issue.
4. Wait the manteiner review and i'm Best scenario, accept Pull Request, if not, i give you a feedback of why was rejected.

### Commits structure 
This repository is based in *Convencional Commits*, but about this specific repo, also had many *scopes* based in depth (like *feat(x/y/z)*), in this depth had the next ones:
- **README**: This is when change a [README.md file](./README.md), used to doucment why is this project as a general description.

- **Cargo.toml**: This is when change a [Cargo.toml file](./Cargo.toml), used to change config of the workspaces.

- **CONTRIBUTING**: This is when change a [CONTRIBUTING.md file](./CONTRIBUTING.md), used to indicate the guide of contributing.