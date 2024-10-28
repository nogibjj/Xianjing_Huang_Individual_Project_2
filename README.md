# Xianjing_Huang_Individual_Project_2
[![CI](https://github.com/nogibjj/Xianjing_Huang_Individual_Project_2/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Xianjing_Huang_Individual_Project_2/actions/workflows/CI.yml)

### Directory Tree Structure
```
Xianjing_Huang_Individual_Project_2/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   └── workflows/
│       └── CI.yml
├── imgs/
├── sqlite/
│   ├── data/
│   │   └── customer_new.csv
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── tests/
│   │   └── test.rs
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Makefile
│   └── my_database.db
├── .gitignore
└── README.md
```
`Cargo.toml`: Package and dependencies.

`sqlite/src/lib.rs`:
- `extract`: Extract a dataset from a url to a file path.
- `create_table`: Create a table.
- `load_data_from_csv`: Load data from a file path to a table.
- `drop_table`: Drop a table.
- `create_exec`: Insert a record in the table.
- `read_exec`: Read records in table.
- `update_exec`: Update a record in the table.
- `delete_exec`: Delete a record in the table.

`sqlite/src/main.rs`: 
Handle CLI features by parsing input as one of 8 options (ETL-CRUD): Extract, Create, Load, Query, Insert, Update, Delete, Drop. 

`sqlite/tests/test.rs`: Test for lib.

`Makefile`: Defines scripts for common project tasks such as cargo check, cargo build. Also defines custom tasks to demonstrate extract, load, and CRUD (Create, Read, Update, Delete) operations.

`CI.yml`: Defines the GitHub Actions workflow for Check, Format, Test, Release, Upload Binary Artifact (Optimized Rust Binary).

### Requirements
* Rust source code: The code should comprehensively understand Rust's syntax and unique features.
* Use of LLM: In your README, explain how you utilized an LLM in your coding process.
* SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
* Optimized Rust Binary: Include a process that generates an optimized Rust binary as a Gitlab Actions artifact that can be downloaded.
* README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how Gitlab Copilot was used.
* Github/Gitlab Actions: A workflow file that tests, builds, and lints your Rust code.
* Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

### Youtube Video Link 
[Youtube Link](https://youtu.be/r5o6zzqpUm8)

### Preparation
1. Open codespaces
2. Install cargo
>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
>cargo --version
3. If running locally, `git clone` the repository and enter WORKING_DIR: sqlite `cd sqlite`.

### Dependencies
Dependencies are in Cargo.toml.
```
[package]
name = "sqlite"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5.20", features = ["derive"] }
csv = "1.3.0"
rusqlite = "0.32.1"
once_cell = "1.10"
reqwest = { version = "0.11", features = ["blocking"] }
```
* Compile and install dependencies: ``cargo build``
* Update all dependencies to the latest compatible versions: ``cargo update``
* List all dependencies: ``cargo tree``


### Build
Rust is a compiled language and so to run programs you will need to compile the file first. This is done a few ways:

>cargo check

* a quick compile that works off of a cached version to only rebuild changes

>cargo build

* an unoptimized build which has debug functionality

>cargo fmt

* automatically formats Rust code according to the standard Rust style guidelines.

>cargo clippy

* It’s a linter that catches all those little things you might overlook—warnings, performance issues, or just some best practice suggestions. 

>cargo test
* is your tool for running tests in your Rust code.

>cargo build --release

* generates an optimized binary in your target/release/\<projectname> directory

![0](/imgs/000.png)

### Project Breakdown
In this project, I use rust to realize SQLite operation and use CLI(Command-Line Tool) features.

In lib.rs, I implement `extract` to extract a dataset to 'data/customer_new.csv'; `load_data_from_csv` to load data into a table; CRUD operations are `create_exec`, `read_exec`, `update_exec`, `delete_exec` for "inserting, reading, updating, deleting" records in the table.

See main.rs for a commented example of how we make our CLI. Note that by using clap over standard library options (std::env for rust or argparse for python) we get a lot of free functionality like help menu guides.

Add compiled binary (--release) to your path, this way you can use your CLI normally without using:

>cargo run -- -\<flag> argument

and instead:

>sqlite -c users

Command to add compiled binary to path for use:

*If in codespaces*

>export PATH=$PATH:/workspaces/\<REPO_NAME>/sqlite/target/release

Once you build your CLI binary you can the use it like a regular CLI:
![1](/imgs/001.png)

#### CRUD demo
![2](/imgs/002.png)
![3](/imgs/003.png)
![4](/imgs/004.png)
`cargo run -- -e`  Extract a dataset to 'data/customer_new.csv'.
`cargo run -- -c table1` Create Table table1.

`cargo run -- -l table1 data/customer_new.csv` Load data into table 'table1' from 'data/customer_new.csv'.

`cargo run -- -q table1` Read all records from table1;

`cargo run -- -i table1 11 Remi female Durham` Insert person with ID '11' into the 'table1' table.

`cargo run -- -u table1 11 Remi female 'Los Angeles'` Updating record in table 'table1' with ID 11.

`cargo run -- -x table1 11` Delete record in table 'table1' with ID 11

`cargo run -- -d table1` Table 'table1' dropped.

### Optimized Rust Binary Download
https://github.com/nogibjj/Xianjing_Huang_Individual_Project_2/actions/runs/11559464823/artifacts/2113666605

The binary location is what gets uploaded as an artifact in the yml file.
![5](/imgs/005.png)

### Continuous Integration (CI/CD Pipeline)

![6](/imgs/006.png)

### Use of Large Language Model (LLM)
#### 1. Syntax
* The LLM provided helpful assistance with Rust syntax, offering clear explanations and examples that made understanding Rust’s unique features, such as **ownership**, **borrowing**, and **strict type system**, much easier. Its guidance simplified the process of writing and debugging Rust code.
#### 2. Error Handling 
* The LLM helps error handling by introducing ``rusqlite::Result``. In rusqlite, the Result type is used to handle potential errors that may arise during database operations. Rust’s Result type is a core part of its error-handling model and provides a way to represent either success (``Ok``) or failure (``Err``) of an operation, which  ensures that each operation is either successful or handled gracefully.
#### 3. Single-Threaded Resource Locking 
* The LLM helped me resolve issues with Rust tests on a data table by guiding me to switch from the default multi-threaded test environment to a single-threaded mode. By using a ``static DB_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));`` to lock access, I was able to prevent errors caused by concurrent access to shared resources, making the tests stable and reliable.

### GitLab Copilot
GitLab Copilot (often referred to as GitLab’s AI-powered DevOps tools) provides features to streamline code development, testing, and collaboration directly within GitLab’s interface.

**Automated Code Suggestions**: GitLab Copilot assists by suggesting improvements, and automating repetitive coding tasks.

**CI/CD Pipeline Automation**: By suggesting pipeline configuration and optimizations, GitLab Copilot assists with setting up and maintaining CI/CD workflows. It can optimize pipelines by identifying redundant jobs, suggesting parallelization, reducing manual setup and maintenance time.
