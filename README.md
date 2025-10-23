Here is the **Markdown file content** for your GitHub repository README as a plain text you can save as `README.md`:

```
# ⚡ Accio CLI

**Accio** is a blazing‑fast, modern, and user‑friendly command‑line tool built in **Rust**.  
It provides functionality similar to Unix file utilities — such as listing, locating, and managing files — using **multi‑threading**, **progress indicators**, and **tiny footprint performance**.

---

## ✨ Features

- Lightweight and compiled with Rust’s zero‑cost abstractions
- Sub‑command architecture using [`clap`](https://github.com/clap-rs/clap)
- Dynamic progress spinners with [`indicatif`](https://github.com/console-rs/indicatif)
- Optional multi‑core parallel search using [`rayon`](https://github.com/rayon-rs/rayon)
- Clean professional CLI with automatic help and version flags
- Reports execution time (e.g., `2m 32secs` format)

---

## 🚀 Installation

### Prerequisites
1. Install **Rust** (via [rustup.rs](https://rustup.rs))
2. Clone this repository:
   ```
   git clone https://github.com/<your-username>/accio.git
   cd accio
   ```
3. Build:
   ```
   cargo build --release
   ```
4. Run:
   ```
   cargo run -- <COMMAND>
   ```

---

## 📖 Usage

### 1. Show Help
```
accio --help
```

### 2. Print Working Directory
```
accio pwd
```

### 3. List Files in Current Directory
```
accio ls
```

### 4. Search for a File
```
accio search <filename>
```
Example:
```
accio search report.txt
```
When executed, you will be prompted for:
```
Enter directory path to scan: D:\Projects
Use parallel search (Rayon)? (y/n): y
Starting parallel search...
✔ Found the following files: (Completed in 1m 34secs)
D:\Projects\Docs\report.txt
```

---

## ⚙️ How It Works

- `clap` provides declarative argument parsing and help.
- `indicatif` displays real‑time scanning progress.
- `rayon` provides optional parallel recursive scanning using all CPU cores.
- Both sequential and parallel modes measure execution time using Rust’s built‑in timers.

---

## 🧩 Commands

| Command | Description |
|----------|--------------|
| `pwd` | Prints the current working directory |
| `ls`  | Lists all files and folders in the current directory |
| `search <filename>` | Recursively searches for the specified filename in a given directory |
| `-h`, `--help` | Displays help information |
| `--version` | Displays the version number |

---

## 🕒 Example Output

```
Enter directory path to scan: C:\Users\Steve\Downloads
Use parallel search (Rayon)? (y/n): n
Starting sequential search...
✔ Found the following files: (Completed in 2m 10secs)
C:\Users\Steve\Downloads\report.txt
```

---

## 🧠 Tech Stack

- **Language:** Rust 1.74+
- **CLI Framework:** Clap (v4.5)
- **Progress Display:** Indicatif
- **Parallel Engine:** Rayon
- **Project Type:** Command‑Line Application

---

## 🧑‍💻 Development Setup

### Debug Run
```
cargo run -- search notes.txt
```

### Release Build
```
cargo build --release
./target/release/accio ls
```

---

## 🪄 Future Enhancements

- `--path` and `--parallel` flags (non‑interactive mode)
- File content text search (like `grep`)
- JSON output mode for integrations
- Cross‑platform packaging (`.exe`, `.deb`, `.tar.gz`)

---

## 📄 License

Licensed under the **MIT License**.  
See the [LICENSE](./LICENSE) file for details.

---

## ❤️ Acknowledgments

Built with Rust, powered by:
- [Clap](https://github.com/clap-rs/clap)
- [Rayon](https://github.com/rayon-rs/rayon)
- [Indicatif](https://github.com/console-rs/indicatif)

---
[8](https://www.w3docs.com/nx/marked)
[9](https://mconverter.eu/convert/markdown/docx/)
[10](https://www.switchlabs.dev/markdown-to-richtext)
