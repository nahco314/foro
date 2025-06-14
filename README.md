<p align="center">
  <img src="https://raw.githubusercontent.com/your-username/foro/main/path/to/your/logo.png" alt="foro Logo" width="200"/>
</p>

<h1 align="center">foro</h1>

<p align="center">
  <strong>The Blazingly Fast, Plugin-Driven Universal Code Formatter.</strong>
</p>

---

> [!WARNING]
> **Experimental project – subject to change**
>
> Foro is under active development.  
> Interfaces and specifications can change without notice, potentially introducing breaking changes.  
> You are welcome to try it, but please proceed with caution.
>
> _Heads-up:_ This README was generated by AI and is still a work-in-progress; it may contain inaccuracies or outdated information.


**Tired of slow formatters? Juggle multiple tools for different languages? Enter `foro` – the next-generation code formatter designed for speed, extensibility, and a seamless developer experience.**

`foro` leverages a powerful daemon process and a smart plugin system to deliver lightning-fast formatting across a multitude of languages. Configure it once, and let `foro` handle the rest, ensuring your codebase stays consistent and beautiful without the wait.

## ✨ Why `foro`?

* 🚀 **Blazing Fast:** Seriously, it's fast. `foro` consistently outperforms popular formatters. Don't believe us? Check the [benchmarks](#-benchmarks)!
* 🧩 **Plugin-Powered & Extensible:**
    * Format *any* language by leveraging `foro`'s plugin architecture.
    * Plugins are distributed as `.dllpack` files and can be fetched from URLs.
    * Integrate existing formatters (like `gofmt`) or build your own custom logic.
    * Supports both WASM and Native plugins for maximum flexibility and performance.
* ⚡ **Daemon Mode:** A persistent background daemon ensures near-instantaneous formatting responses.
* 🧠 **Smart Caching:** Efficiently caches plugins and formatting results to minimize redundant work and maximize speed.
* 🔧 **Effortless Configuration:** A simple `foro.json` file is all you need to define your formatting rules and plugins.
* параллельная обработка (Parallel Processing): Optimised for multi-core processors to format files in bulk efficiently.
* 🛡️ **Robust & Reliable:** Built with Rust for safety and performance. Includes features like startup locks and build ID mismatch detection for daemon stability.

## 🤯 Benchmarks

`foro` is engineered for speed. Here's how it stacks up against some well-known formatters:

*(Benchmark environment: [Specify your environment if possible, otherwise omit or use a generic placeholder])*

**Key Takeaways:**

* **Biome (TypeScript/JavaScript):**
    * Up to **~139.85x faster** than `npx biome format` for small files.
    * Up to **~41.42x faster** than `npx biome format` for large files.
    * Up to **~2.52x faster** than direct Biome CLI execution for small files.
    * Up to **~1.90x faster** than direct Biome CLI execution for large files.
* **Ruff (Python):**
    * Up to **~3.40x faster** than `ruff format` for small files.
    * Up to **~1.69x faster** than `ruff format` for large files.
* **Clang-Format (C/C++):**
    * Up to **~6.83x faster** than `clang-format` for small files.
    * Up to **~2.01x faster** than `clang-format` for large files.

<details>
<summary><strong>Click to see detailed benchmark results</strong></summary>

```text
Running benchmarks...

Running benchmark for biome + small...
Benchmark 1: npx biome format --write ./src/small.tsx
  Time (mean ± σ):       293391.7 µs ± 10105.9 µs    [User: 271435.0 µs, System: 65065.8 µs]
  Range (min … max):   284635.4 µs … 308437.9 µs    10 runs

Benchmark 2: ./node_modules/@biomejs/cli-linux-x64-musl/biome format --write ./src/small.tsx
  Time (mean ± σ):       5294.7 µs ± 155.1 µs    [User: 2129.8 µs, System: 4882.4 µs]
  Range (min … max):   4924.6 µs … 6024.6 µs    497 runs

Benchmark 3: foro format ./src/small.tsx
  Time (mean ± σ):       2097.9 µs ±  85.0 µs    [User: 1034.7 µs, System: 503.9 µs]
  Range (min … max):   1977.2 µs … 2459.7 µs    1275 runs

Summary
  foro format ./src/small.tsx ran
    2.52 ± 0.13 times faster than ./node_modules/@biomejs/cli-linux-x64-musl/biome format --write ./src/small.tsx
  139.85 ± 7.44 times faster than npx biome format --write ./src/small.tsx

Running benchmark for biome + large...
Benchmark 1: npx biome format --write ./src/large.tsx
  Time (mean ± σ):       307042.1 µs ± 5912.9 µs    [User: 270977.7 µs, System: 71831.9 µs]
  Range (min … max):   292327.5 µs … 312775.1 µs    10 runs

Benchmark 2: ./node_modules/@biomejs/cli-linux-x64-musl/biome format --write ./src/large.tsx
  Time (mean ± σ):       14064.3 µs ± 329.6 µs    [User: 8886.1 µs, System: 6946.3 µs]
  Range (min … max):   13494.4 µs … 15509.6 µs    211 runs

Benchmark 3: foro format ./src/large.tsx
  Time (mean ± σ):       7413.4 µs ± 194.2 µs    [User: 1101.2 µs, System: 561.4 µs]
  Range (min … max):   7097.3 µs … 8451.8 µs    354 runs

Summary
  foro format ./src/large.tsx ran
    1.90 ± 0.07 times faster than ./node_modules/@biomejs/cli-linux-x64-musl/biome format --write ./src/large.tsx
   41.42 ± 1.35 times faster than npx biome format --write ./src/large.tsx

Running benchmark for ruff format + small...
Benchmark 1: ruff format ./src/ruff_test/small.py
  Time (mean ± σ):       7643.1 µs ± 244.2 µs    [User: 3008.0 µs, System: 10613.1 µs]
  Range (min … max):   7087.1 µs … 8777.2 µs    370 runs

Benchmark 2: foro format ./src/ruff_test/small.py
  Time (mean ± σ):       2246.6 µs ± 142.5 µs    [User: 1080.1 µs, System: 551.2 µs]
  Range (min … max):   2001.4 µs … 3892.2 µs    1374 runs

Summary
  foro format ./src/ruff_test/small.py ran
    3.40 ± 0.24 times faster than ruff format ./src/ruff_test/small.py

Running benchmark for ruff format + large...
Benchmark 1: ruff format ./src/ruff_test/large.py
  Time (mean ± σ):       7731.6 µs ± 288.0 µs    [User: 2964.2 µs, System: 10826.5 µs]
  Range (min … max):   7115.5 µs … 9106.5 µs    380 runs

Benchmark 2: foro format ./src/ruff_test/large.py
  Time (mean ± σ):       4584.0 µs ± 218.3 µs    [User: 1220.0 µs, System: 483.6 µs]
  Range (min … max):   4141.3 µs … 5686.4 µs    642 runs

Summary
  foro format ./src/ruff_test/large.py ran
    1.69 ± 0.10 times faster than ruff format ./src/ruff_test/large.py

Running benchmark for clang-format + small...
Benchmark 1: clang-format ./small.cpp
  Time (mean ± σ):       16944.7 µs ± 776.9 µs    [User: 5167.3 µs, System: 11704.4 µs]
  Range (min … max):   16277.1 µs … 19471.0 µs    165 runs

Benchmark 2: foro format ./small.cpp
  Time (mean ± σ):       2481.7 µs ± 123.0 µs    [User: 1074.5 µs, System: 549.7 µs]
  Range (min … max):   2264.6 µs … 2848.4 µs    1088 runs

Summary
  foro format ./small.cpp ran
    6.83 ± 0.46 times faster than clang-format ./small.cpp

Running benchmark for clang-format + large...
Benchmark 1: clang-format ./large.cpp
  Time (mean ± σ):       24076.3 µs ± 1070.5 µs    [User: 10830.1 µs, System: 13172.8 µs]
  Range (min … max):   23150.1 µs … 30047.2 µs    123 runs

Benchmark 2: foro format ./large.cpp
  Time (mean ± σ):       11975.7 µs ± 355.8 µs    [User: 1120.1 µs, System: 442.1 µs]
  Range (min … max):   11567.6 µs … 13565.9 µs    234 runs

Summary
  foro format ./large.cpp ran
    2.01 ± 0.11 times faster than clang-format ./large.cpp
````

</details>

## 🚀 Getting Started

### Installation

*(Provide installation instructions here. E.g., using `cargo`, pre-compiled binaries, package managers, etc.)*

**Example (Cargo):**

```bash
cargo install foro
```

**Pre-compiled binaries:**
Check the [Releases page](https://www.google.com/search?q=https://github.com/your-username/foro/releases) for pre-compiled binaries for your system.

### Basic Usage

Once installed, `foro` is easy to use. It relies on a daemon process for speed. The daemon will be started automatically if not running.

**1. Formatting a Single File:**

```bash
foro format path/to/your/file.ext
```

`foro` will automatically detect the file type and apply the configured formatter.

**2. Formatting Multiple Files / Directories (Bulk Format):**

```bash
# Format all files in the current directory and subdirectories
foro bulk-format .

# Format specific paths
foro bulk-format ./src ./docs

# Specify number of threads (defaults to number of CPU cores)
foro bulk-format --threads 4 ./my_project
```

## 🛠️ Configuration

`foro` is configured using a `foro.json` file. By default, `foro` looks for this file in your system's configuration directory. You can also specify a custom path using the `--config-file` option.

A default configuration file will be created automatically if one doesn't exist.

**Default Configuration Location:**

* Linux: `~/.config/foro/foro.json`
* macOS: `~/Library/Application Support/foro/foro.json`
* Windows: `C:\Users\YourUser\AppData\Roaming\foro\foro.json`

You can find the exact path using:

```bash
foro config path
```

### `foro.json` Structure

The configuration file consists of an array of `rules`. Each rule defines:

* `on`: File extension(s) the rule applies to (e.g., `".ts"`, `[".js", ".tsx"]`).
* `cmd` or `write_cmd`: The command to execute.
  * `cmd`: For "pure" commands that return the formatted content as a string (e.g., plugins that output to stdout). `foro` will then write this to the file if changes are detected.
    * Can be a URL to a `.dllpack` plugin: `"cmd": "https://example.com/my-formatter.dllpack"`
    * Can be an I/O command: `"cmd": { "io": "gofmt" }` (takes input via stdin, outputs to stdout)
  * `write_cmd`: For commands that write directly to the file system (e.g., `rustfmt {{ os-target }}`).

**Example `default_config.json` (snippet):**

```json
{
	"rules": [
		{
			"on": [".ts", ".tsx", ".js", ".mjs", ".cjs", ".json"],
			"cmd": "[https://github.com/nahco314/foro-biome/releases/download/0.5.1/foro-biome.dllpack](https://github.com/nahco314/foro-biome/releases/download/0.5.1/foro-biome.dllpack)"
		},
		{
			"on": ".rs",
			"cmd": "[https://github.com/nahco314/foro-rustfmt/releases/download/0.4.1/foro-rustfmt.dllpack](https://github.com/nahco314/foro-rustfmt/releases/download/0.4.1/foro-rustfmt.dllpack)"
		},
		{
			"on": ".py",
			"cmd": "[https://github.com/nahco314/foro-ruff/releases/download/0.3.1/foro-ruff.dllpack](https://github.com/nahco314/foro-ruff/releases/download/0.3.1/foro-ruff.dllpack)"
		},
		{
			"on": ".go",
			"cmd": {
				"io": "gofmt" // Uses the system's gofmt command
			}
		},
		{
			"on": [".c", ".cpp", ".h", ".hpp"],
			"cmd": "[https://github.com/nahco314/foro-clang-format/releases/download/0.2.0/foro-clang-format.dllpack](https://github.com/nahco314/foro-clang-format/releases/download/0.2.0/foro-clang-format.dllpack)"
		}
	]
}
```

To see the full default configuration:

```bash
foro config default
```

## 🔌 Plugins

`foro`'s power comes from its plugin system. Plugins are typically `.dllpack` files which can be either WebAssembly (WASM) modules or native shared libraries.

* **Distribution:** Plugins are fetched from URLs specified in your `foro.json`.
* **Execution:** `foro` manages the download, caching, and execution of these plugins.
* **Interface:** Plugins implement a simple `foro_main` function that receives formatting information (like file path and content) as a JSON string and returns results similarly.
* **Caching:** Downloaded plugins are cached efficiently. `foro` also uses a sophisticated multi-resource cache (`ResourcePool`) to allow concurrent, non-blocking access to plugin instances from multiple threads, crucial for `bulk-format`.

This system allows `foro` to be extended to support virtually any formatting tool or custom logic.

## 💻 CLI Commands

Here's a quick overview of `foro`'s commands:

* `foro format <path>`: Formats a single file.
* `foro bulk-format [paths...] [--threads N]`: Formats multiple files or directories.
* **Daemon Management (`foro daemon ...`):**
  * `start [--attach]`: Starts the daemon (detaches by default).
  * `stop`: Stops the daemon.
  * `restart [--attach]`: Restarts the daemon.
  * `ping`: Checks if the daemon is running and shows info.
* **Configuration Management (`foro config ...`):**
  * `path`: Shows the path to the current configuration file.
  * `show`: Displays the content of the current configuration file.
  * `default`: Prints the default configuration.
* **Cache Management (`foro cache ...`):**
  * `clean [--yes]`: Clears the entire `foro` cache directory.
  * `remove <url>`: Removes cache for a specific plugin URL.
  * `dir`: Shows the path to the cache directory.

**Global Options:**

* `--config-file <PATH>`: Path to a custom `foro.json` file.
* `--cache-dir <PATH>`: Path to a custom cache directory.
* `--socket-dir <PATH>`: Path to a custom directory for the daemon socket.
* `--no-cache`: Disables reading from or writing to the cache for the current command.
* `--verbose (-v, -vv, ...)`: Increases log verbosity.
* `--long-log`: Enables more detailed log format.
* `--ignore-build-id-mismatch`: Prevents daemon restart if client and daemon have different build IDs.

For detailed help on any command, use `foro <command> --help`.

## 🤝 Contributing

Contributions are welcome\! Whether it's bug reports, feature requests, documentation improvements, or code contributions, please feel free to open an issue or pull request.

Please read our [CONTRIBUTING.md](https://www.google.com/search?q=https://github.com/your-username/foro/blob/main/CONTRIBUTING.md) (you'll need to create this file) for guidelines.

## 📜 License

`foro` is licensed under the [Your License Type, e.g., MIT License or Apache 2.0] - see the [LICENSE](https://www.google.com/url?sa=E&source=gmail&q=https://github.com/your-username/foro/blob/main/LICENSE) file for details.
*(You'll need to add a LICENSE file to your repository).*

-----

<p align="center">
<em>Format with foro, Format with FURY\!</em> ⚡
</p>
