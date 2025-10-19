# pouet2excel

This program uses data from [https://data.pouet.net/](https://data.pouet.net/) to generate an Excel file with information about **demoscene productions**. Every **Wednesday at 20:00 CET**, the project runs automatically on GitHub Actions and publishes the latest generated Excel file on the [GitHub Pages site](https://nagyattis81.github.io/pouet2excel/).

---

## 🦀 Requirements

- [Rust](https://www.rust-lang.org/) (stable toolchain)
- Internet access for downloading the Pouët dataset

---

## 🏗️ Build & Run

To build and run the program locally:

```bash
cargo run --bin pouet2excel
```

Example output when running for the first time:

```bash
✅ Read https://data.pouet.net/json.php
✅ Download pouetdatadump-prods-20251015.json.gz
✅ Decompress pouetdatadump-prods-20251015.json
✅ Write excel pouetdatadump-prods-20251015.json.xlsx
```

If the local pouetdatadump-prods-\*.json file already exists,
it will be reused instead of downloading again:

```bash
✅ Read https://data.pouet.net/json.php
ℹ️  Found existing file: pouetdatadump-prods-20251015.json
✅ Write excel pouetdatadump-prods-20251015.json.xlsx
```

## ⚙️ How It Works

- It downloads the latest Pouët dataset (.json.gz).
- The data is decompressed and converted to .xlsx format.

## 🔗 Related Links

- [pouet.net](https://www.pouet.net/)
- [pouet.net Data API](https://data.pouet.net/)
- [demoscene on Wikipedia](https://hu.wikipedia.org/wiki/Demoscene)
- [Vakondok 2 - Demoscene](https://www.youtube.com/watch?v=Z-keHkcTZD4)

## 🧾 License

This project is licensed under the MIT License.
See LICENSE for details.
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
