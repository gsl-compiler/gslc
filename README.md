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
Then follow the instructions given by the compiler to add it to your bin and use it.

**Usage**
**Translate shorthand:**
```bash
gslc '\\P:A/P:B/S:AB\\'
```
**Save to file:**
```bash
gslc '\\P:A/P:B/S:AB\\' -o output.txt
```
**From file:**
```bash
gslc -f input.gsl -o output.txt
```
**Examples**
```bash
# Simple construction
gslc '\\P:A,B/S:AB/R:3;AB=ABC/[ABC]?\\'

# Complex problem
gslc '\\P:A,B/S:AB/R:3;AB=ABC/P:D.AC|R:3;AD=ADE,[ADE]=20/[DFH]?\\'
```
**Links**
* Online Translator: https://politikl.github.io/geometry-shorthand-translator
* Documentation: https://tinyurl.com/geoshorthand
* GitHub: https://github.com/politikl/gslc


