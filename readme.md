
<div align="center">
<h1 align="center">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" />
<br>cpf_validator
</h1>
<h3>◦ Cpf_validator: Simplifying CPF validation effortlessly</h3>
<h3>◦ Developed with the software and tools listed below.</h3>

<p align="center">
<img src="https://img.shields.io/badge/Rust-000000.svg?style&logo=Rust&logoColor=white" alt="Rust" />
</p>
<img src="https://img.shields.io/github/languages/top/chmenegatti/cpf_validator.git?style&color=5D6D7E" alt="GitHub top language" />
<img src="https://img.shields.io/github/languages/code-size/chmenegatti/cpf_validator.git?style&color=5D6D7E" alt="GitHub code size in bytes" />
<img src="https://img.shields.io/github/commit-activity/m/chmenegatti/cpf_validator.git?style&color=5D6D7E" alt="GitHub commit activity" />
<img src="https://img.shields.io/github/license/chmenegatti/cpf_validator.git?style&color=5D6D7E" alt="GitHub license" />
</div>

---

## 📒 Table of Contents
- [📒 Table of Contents](#-table-of-contents)
- [📍 Overview](#-overview)
- [⚙️ Features](#-features)
- [📂 Project Structure](#project-structure)
- [🧩 Modules](#modules)
- [🚀 Getting Started](#-getting-started)
- [🗺 Roadmap](#-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [👏 Acknowledgments](#-acknowledgments)

---


## 📍 Overview

The core functionality of the project is to validate Brazilian CPF (Cadastro de Pessoas Físicas) numbers entered by the user. It removes any non-numeric characters from the input, checks the length of the CPF (which should be 11 digits), and performs calculations to verify its validity. The project's purpose is to ensure that the CPF numbers provided are accurate and can be used for identification purposes. This provides value by helping to prevent errors or fraudulent use of CPF numbers.

---

## ⚙️ Features

| Feature                | Description                           |
| ---------------------- | ------------------------------------- |
| **⚙️ Architecture**     | The codebase follows a simple procedural design where the logic for validating CPF is implemented in a single main.rs file. The code removes non-numeric characters from the user input, checks for a length of 11, and calculates two verification values. The verification values are then compared to the last two digits of the CPF to determine its validity. The architecture does not adhere to any specific design patterns or architectural decisions. It can be considered as a basic implementation.    |
| **📖 Documentation**   | The codebase lacks comprehensive documentation. Only a brief description of the main.rs file's functionality is provided in the repository's README. The absence of detailed comments or explanatory documentation within the code makes it challenging for developers to understand the implementation and potential use cases. Improving the documentation would enhance the project's maintenance and usability.    |
| **🔗 Dependencies**    | The codebase has no external dependencies. It relies solely on the standard Rust library, which reduces the complexity of the project and simplifies the setup for developers.    |
| **🧩 Modularity**      | The codebase does not appear to be organized into separate, reusable components. All the logic for validating the CPF is implemented in the single main.rs file. This lack of modularity makes it difficult to reuse or test specific parts of the codebase independently. Refactoring the code to separate concerns into smaller modules or functions would improve maintainability and reusability.    |
| **✔️ Testing**          | The codebase does not include any automated tests. The absence of test cases makes it challenging to ensure the correctness and reliability of the CPF validation logic. Implementing unit tests, integration tests, or even property-based tests using a testing framework like `rusty-fork` or `quickcheck` would greatly enhance the code quality and enable easier refactoring in the future.    |
| **⚡️ Performance**      | The codebase performs well in terms of speed and resource usage. Since it only performs straightforward calculations on a small input (CPF), the computational complexity is minimal. The codebase does not exhibit any known performance issues, and it efficiently utilizes system resources.    |
| **🔐 Security**        | The codebase does not incorporate specific measures to ensure security. It only focuses on validating the structure of the CPF. However, input validation alone is not sufficient to guarantee overall security. To improve security, additional measures like input sanitization, output encoding, and proper error handling should be implemented to prevent potential security vulnerabilities such as input injection or information disclosure.    |
| **🔀 Version Control** | The codebase utilizes Git for version control. It is hosted on GitHub, which provides a robust version control system with features like branching, committing, and merging. However, the repository does not include any specific version control strategies or

---


## 📂 Project Structure




---

## 🧩 Modules

<details closed><summary>Src</summary>

| File                                                                              | Summary                                                                                                                                                                                                                                                                                                                                                |
| ---                                                                               | ---                                                                                                                                                                                                                                                                                                                                                    |
| [main.rs](https://github.com/chmenegatti/cpf_validator.git/blob/main/src/main.rs) | This code snippet validates a CPF (Brazilian ID) entered by the user. It removes non-numeric characters, checks for a length of 11, and calculates two verification values. The verification values are compared to the last two digits of the CPF to determine its validity. If valid, it prints "CPF válido!"; otherwise, it prints "CPF inválido!". |

</details>

---

## 🚀 Getting Started

### ✔️ Prerequisites

Before you begin, ensure that you have the following prerequisites installed:
> - `ℹ️ Requirement 1`
> - `ℹ️ Requirement 2`
> - `ℹ️ ...`

### 📦 Installation

1. Clone the cpf_validator repository:
```sh
git clone https://github.com/chmenegatti/cpf_validator.git
```

2. Change to the project directory:
```sh
cd cpf_validator
```

3. Install the dependencies:
```sh
cargo build
```

### 🎮 Using cpf_validator

```sh
cargo run
```

### 🧪 Running Tests
```sh
cargo test
```

---


## 🗺 Roadmap

> - [X] `ℹ️  Task 1: Implement X`
> - [ ] `ℹ️  Task 2: Refactor Y`
> - [ ] `ℹ️ ...`


---

## 🤝 Contributing

Contributions are always welcome! Please follow these steps:
1. Fork the project repository. This creates a copy of the project on your account that you can modify without affecting the original project.
2. Clone the forked repository to your local machine using a Git client like Git or GitHub Desktop.
3. Create a new branch with a descriptive name (e.g., `new-feature-branch` or `bugfix-issue-123`).
```sh
git checkout -b new-feature-branch
```
4. Make changes to the project's codebase.
5. Commit your changes to your local branch with a clear commit message that explains the changes you've made.
```sh
git commit -m 'Implemented new feature.'
```
6. Push your changes to your forked repository on GitHub using the following command
```sh
git push origin new-feature-branch
```
7. Create a new pull request to the original project repository. In the pull request, describe the changes you've made and why they're necessary.
The project maintainers will review your changes and provide feedback or merge them into the main branch.

---

## 📄 License

This project is licensed under the `ℹ️  INSERT-LICENSE-TYPE` License. See the [LICENSE](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/adding-a-license-to-a-repository) file for additional info.

---

## 👏 Acknowledgments

> - `ℹ️  List any resources, contributors, inspiration, etc.`

---
