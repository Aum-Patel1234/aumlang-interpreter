# AumLang Interpreter

**AumLang Interpreter** is a small, interpreter in **C**. (Or so as i thought at first)

- I rewrote it in Rust cause in C i was just constantly thinking about memory, raw dogging \0 at
end of each char[] to terminate string and also use after free errors.
- C was a great experience after all, but then i reasessed my approach and thought
maybe i choose to learn to make interpreter rather than fighting Segfaults.
- i wrote working parser for expressions like (8*9+2) in C using pratt parser. its in C dir.
- maybe in future i will use C when its for the best.
(Side Note) -> Rust enums are the greatest.

It is built as a learning project to explore how interpreted programming languages work.

---

## 🔧 Dependencies

- **C compiler** (GCC or Clang)
- **CMake ≥ 3.16**
- **GLib 2.0**

### Install dependencies (Ubuntu / Debian)

```bash
sudo apt install build-essential cmake libglib2.0-dev
```

---
```bash
git clone <repo-url>
cd aumlang
mkdir build
cd build
cmake ..
make
./aumlang
```

---

### Editor Support

```bash
ln -sf build/compile_commands.json .
```

---

### Address Sanitizer

```bash
valgrind ./aumlang ../file.aum
valgrind --leak-check=full --show-leak-kinds=all ./aumlang ../file.aum
```
