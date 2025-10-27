# GSLC - Geometry Shorthand Language Compiler

A fast, cross-platform compiler for translating Geometry Shorthand Language (GSL) notation into plain English.

## Installation

**macOS/Linux:**
```bash
curl -sSf https://raw.githubusercontent.com/politikl/gslc/main/install.sh | sh
```

**Windows (PowerShell):**
```bash
iwr -useb https://raw.githubusercontent.com/politikl/gslc/main/install.ps1 | iex
```

Then follow the instructions given by the compiler to add it to your binaries and use!

## Features

- 🚀 **Fast Translation**: Instantly convert GSL shorthand to readable English
- 🗣️ **Pronunciation Guide**: Learn how to pronounce GSL shorthand
- 📁 **File Support**: Load and save `.gsl` files
- 🔄 **Step-by-Step Mode**: Break down pronunciations by statement
- 🌐 **Cross-Platform**: Works on Windows, macOS, and Linux
- 📚 **Complete GSL Support**: Updated with latest language specification

## Usage

### Basic Translation

**Translate shorthand directly:**
```bash
gslc '\\P:A/P:B/S:AB\\'
```

Output:
```
1. Construct points A, B.
2. Connect segment AB.
```

**Save to file:**
```bash
gslc '\\P:A/P:B/S:AB\\' -o output.txt
```

**Load from file:**
```bash
gslc -f problem.gsl
```

**Load and save:**
```bash
gslc -f problem.gsl -o solution.txt
```

### Pronunciation Mode

**One-line pronunciation:**
```bash
gslc --pron '\\P:A/S:AB\\'
```

Output:
```
p kuh a mn s kuh a b
```

**Step-by-step pronunciation:**
```bash
gslc --pron -s '\\P:A/S:AB\\'
```

Output:
```
1. p kuh a
2. s kuh a b
```

**Pronounce from file:**
```bash
gslc --pron -f sample.gsl
```

**Step-by-step from file:**
```bash
gslc --pron -s -f sample.gsl
```

### Help Commands

**View help:**
```bash
gslc help
```

**View about info:**
```bash
gslc about
```

**Open language documentation:**
```bash
gslc lang
```

## Examples

### Simple Construction
```bash
gslc '\\P:A,B/S:AB/R:3;AB=ABC/[ABC]?\\'
```

Output:
```
1. Construct points A, B.
2. Connect segment AB.
3. Construct equilateral triangle ABC with side AB.
4. What is the area of ABC?
```

### Complex Problem
```bash
gslc '\\P:A,B/S:AB/R:3;AB=ABC/P:D.AC|R:3;AD=ADE,[ADE]=20/P:F.BC|R:3;BF=BFG,[BFG]=5/S:DF/P:H.AB|J:DFH*R/[DFH]?\\'
```

### Using Sample File
A sample GSL file is included:
```bash
gslc -f sample.gsl
```

## GSL Quick Reference

### Constructions
- `P:A` - Construct point A
- `S:AB` - Connect segment AB
- `L:AB` - Connect line AB
- `W:AB` - Construct ray AB
- `C:O;r` - Circle with center O and radius r
- `J:ABC` - Construct polygon ABC
- `R:n;AB=POLYGON` - Regular n-gon

### Queries
- `[ABC]?` - What is the area of ABC?
- `(ABC)=x` - Perimeter of ABC is x
- `<ABC=90` - Angle ABC is 90 degrees
- `AB=BC\?` - Prove that AB = BC

### Special Notations
- `P:C..ABC` - Point C in region ABC
- `P:C.AB` - Point C on AB
- `ABC*+` - Points go clockwise
- `ABC*-` - Points go counterclockwise
- `w` prefix - Ray (wAB)
- `l` prefix - Line (lAB)
- `c` prefix - Circle (cO)
- `a` prefix - Arc (aAB)
- `q` prefix - Sector (qOAB)

### Pronunciation Guide
- `:` → "kuh"
- `;` → "suh"
- `,` → "muh"
- `.` → "duh"
- `?` → "kwuh"
- `=` → "eh"
- `|` → "shuh"
- `*` → "xing"
- `x` → "ix"
- `/` → "mn"
- `\\` → "uh"
- `[` → "area"
- `(` → "pairim"
- `w` → "ray"
- `l` → "line"
- `c` → "circ"
- `a` → "arc"
- `q` → "sect"

## Version History

### v1.1.0 (Latest)
- ✨ Added pronunciation mode (`--pron`)
- 📝 Step-by-step pronunciation with `-s` flag
- 📂 Enhanced file loading support
- 🔄 Updated to latest GSL specification
- 🎯 Added bounded area support (`..`)
- ↻ Added rotational specification (`+`/`-`)
- 🏷️ Added new derived constructions (9O, 9C, TG)
- 🧮 Added Pythagorean Theorem (_PY)

### v1.0.0
- 🎉 Initial release
- 🔤 Basic GSL to English translation
- 📁 File input/output support
- 🌐 Cross-platform support

## Links

* **Online Translator**: https://politikl.github.io/geometry-shorthand-translator
* **Language Documentation**: https://tinyurl.com/gsldocumentation
* **GitHub Repository**: https://github.com/politikl/gslc

## Credits

- **Compiler**: politikl
- **GSL Language**: LX and YY

## License

MIT License - See LICENSE file for details

---

*"Geometry Shorthand (Construction) is a code-based language. Complete rigorosity and logicosity is required." - LX*

