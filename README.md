# 🌟 rusty-plot

A tiny, terminal-based graph plotter written in 100% Rust. Plot `y = mx + b` on a 2D ASCII cartesian plane, because why not make your terminal smarter?

---

## 🚀 Overview

rusty-plot lets you visualize linear equations of the form:

```math
y = mx + b
```

right in your terminal, using ASCII art.

It maps a 2D plane from `(-25, -25)` to `(25, 25)` and plots your line, complete with x and y axes.

---

## 🛠 Features

- Render straight lines like it’s 1983 🖥️
- Customizable equations (well... as long as they're `y = mx + b`)
- Shows axes and origin with clean markings
- Totally overengineered for something this simple (and proud of it)

---

## 🧪 Example

```bash
cargo run
```

With this in main.rs:

```rust
graph.equation = "y = -x + 0".to_string();
```

You'll get:

```
            *                     
             *                   
              *                 
               *               
----------------+---------------
                 *               
                  *                 
                   *                   
                    *                     
```

(Just a small piece of the full graph—check your terminal for the full experience!)

---

## 📦 Installation

1. Make sure you have Rust installed. If not:

```bash
curl https://sh.rustup.rs -sSf | sh
```

2. Clone the repo:

```bash
git clone https://github.com/Muchangi001/rusty-plot.git
cd rusty-plot
cargo run
```

---

## 📚 How It Works

- The equation is parsed (assumes form `y = mx + b`)
- x inputs are set from -25 to +25
- y values are calculated
- Points are plotted on a 51x51 grid (centered at [25][25])
- ASCII characters represent:
  - `*` → plotted points
  - `-` → axis
  - `+` → origin

---

## 🧠 Future Goals

- [ ] Support other types of equations (quadratic, exponential, etc.)
- [ ] Add command-line interface for equations
- [ ] Animate plotting step-by-step
- [ ] Add color (if terminal supports ANSI)
- [ ] Turn into a learning playground for high school math and Rust basics

---

## 👨‍💻 Contributing

Pull requests are welcome! This is a fun project for learning, so feel free to fork, hack, and PR.

---

## 📜 License

MIT License. Use it, break it, make it better.

---
