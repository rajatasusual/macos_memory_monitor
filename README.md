# macOS Memory Monitor

A command-line utility written in Rust that lets you to monitor and gain insights into the memory usage and resource consumption of processes running on your macOS system.

## Table of Contents

* [Features](#features)
* [Getting Started](#getting-started)
* [Usage](#usage)
* [Dependencies](#dependencies)
* [Project Structure](#project-structure)
* [Implementation Details](#implementation-details)
* [Key Algorithms & Techniques](#key-algorithms-&-techniques)
* [Future Enhancements](#future-enhancements)
* [Contributing](#contributing)
* [License](#license)
* [Author](#author)

## Features

* **Comprehensive Process Listing:**
    * Retrieves and displays a list of all active processes on your macOS system.
    * Presents essential process information:
        * Process ID (PID): The unique identifier for each process.
        * Process Name: The name of the executable associated with the process.
        * Memory Usage: The amount of memory currently utilized by the process.
        * Total CPU Time: The total CPU time consumed by the process.

* **Intuitive Autocompletion:**
    * Streamlines process selection by providing autocompletion suggestions as you type.
    * Supports autocompletion based on both process names and PIDs.

* **Detailed Process Insights:**
    * Enables you to delve into the specifics of a chosen process, revealing its details in a formatted table:
        * PID
        * Name
        * Memory Usage
        * Total CPU Time

## Getting Started

1. **Prerequisites:**
    * Rust: Ensure you have Rust and Cargo installed on your macOS system. If not, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. **Clone the Repository:**
    ```bash
    git clone [https://github.com/rajatasusual/macos_memory_monitor.git](https://github.com/rajatasusual/macos_memory_monitor.git)
    ```

3. **Build and Run:**
    ```bash
    cd macos_memory_monitor
    cargo build --release 
    ./target/release/macos_memory_monitor
    ```

## Usage

1. **Launch:** Execute the compiled binary to start the macOS Memory Monitor.

2. **Search & Select:**
    * Begin typing a process name or PID. The tool will dynamically offer autocompletion suggestions.
    * Press `Enter` to choose a process and view its comprehensive details.

3. **Exit:**
    * Press `CTRL-C` or `CTRL-D` to gracefully terminate the macOS Memory Monitor.

## Dependencies

This project utilizes the following Rust crates:

* `libc`: Provides bindings to the standard C library, essential for low-level system interactions.
* `libproc`: Facilitates interaction with process information on macOS and Linux systems.
* `prettytable-rs`: Enables the creation of formatted tables for clear and organized output.
* `rustyline`: Empowers the creation of interactive command-line interfaces, incorporating features like history and autocompletion.
* `strsim`: Implements string similarity algorithms, notably the Jaro-Winkler distance, for efficient process matching.

## Project Structure

* `main.rs`:
    * The main entry point of the program.
    * Handles process list initialization, command-line interface setup, and user interaction loop.
    * Orchestrates the retrieval and display of process information.

* `util.rs`:
    * Houses utility functions and structs.
    * `ByteSize`: Provides formatting for memory usage in human-readable units.
    * `TimeFormat`: Provides formatting for CPU time in human-readable units
    * `ProcessCompleter`: Implements autocompletion functionality for process names and PIDs.

## Implementation Details

* **Process Information Retrieval:**
    * Leverages the `libproc` crate to interact with the process table on macOS.
    * Employs `proc_pid::pidinfo` to access process information, including memory usage and CPU time
    * `proc_pid::name` to get process name

* **Autocompletion:**
    * Utilizes the `rustyline` crate to provide interactive command-line features, including autocompletion.
    * The `ProcessCompleter` struct implements the necessary logic to suggest completions based on process names and PIDs.

* **Process Matching:**
    * Employs the Jaro-Winkler similarity algorithm (`strsim` crate) for fuzzy matching of process names.
    * Prioritizes exact PID matches over name matches when both are provided.

* **Output Formatting:**
    * Uses `prettytable-rs` to present process information in a well-structured and visually appealing table format.
    * Employs `prettytable-rs` macros to add basic styling (bold text) to the table headers and attribute names

* **Error Handling:**
    * Makes extensive use of Rust's `Result` type for error handling and propagation.
    * Provides informative error messages to the user when process information retrieval or other operations fail.

## Key Algorithms & Techniques

* **Jaro-Winkler Similarity:**  Measures the similarity between two strings, used for fuzzy matching of process names.
* **Autocompletion:** Implements prefix-based suggestions for process names and PIDs to enhance user experience.

## Future Enhancements

* **Filtering and Sorting:** Allow users to filter and sort the process list based on various criteria.
* **Interactive Mode:**  Enable process selection and perform actions like killing, suspending, or adjusting process priority.
* **Real-Time Updates:**  Periodically refresh the process list and displayed information.
* **Cross-Platform Compatibility:**  Abstract system-specific code to support other operating systems.

## Contributing

Contributions to enhance the macOS Memory Monitor are highly encouraged! Please feel free to:

* Submit pull requests to introduce new features, improvements, or bug fixes.
* Open issues to report bugs, suggest enhancements, or discuss potential future directions.

## License

This project is open-source and available under the MIT License.

## Author

* Rajat [rajatasusual](https://github.com/rajatasusual)