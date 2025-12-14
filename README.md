# AumLang

**AumLang** is a small, interpreted programming language written in **C**.  
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