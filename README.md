# Plonky3 Learning Examples

A collection of practical, working examples to learn [Plonky3](https://github.com/Plonky3/Plonky3) - Polygon's next-generation STARK-based zero-knowledge proof toolkit.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 🎯 What This Repository Contains

This repository provides hands-on examples that demonstrate how to build STARK proof systems using Plonky3, progressing from basic concepts to more advanced patterns.

### Examples Included

| Example | Difficulty | Concepts Covered |
|---------|-----------|------------------|
| [Simple Arithmetic Circuit](#-example-1-simple-arithmetic-circuit) | Beginner | Single-row constraints, basic AIR, trace generation |
| [Fibonacci Sequence](#-example-2-fibonacci-sequence) | Intermediate | Multi-row constraints, state transitions, sequential computation |

---

## 📚 Learning Path

### Start Here: Understanding Plonky3

Before diving into the code, we recommend reading our comprehensive guides:

- **[Plonky3 Beginner's Guide](https://www.notion.so/your-link-here)** - Complete introduction to Plonky3 concepts
- **[Plonky2 vs Plonky3 Comparison](https://www.notion.so/your-link-here)** - Understanding the architectural shift

### Progression

1. **Start with Simple Arithmetic** → Learn the fundamentals of AIR constraints
2. **Move to Fibonacci** → Understand state transitions and multi-row constraints
3. **Explore the guides** → Deepen your understanding of why Plonky3 works this way

---

## 🧮 Example 1: Simple Arithmetic Circuit

**Proves:** `a + c * d = e`

### What You'll Learn

- How to define AIR (Algebraic Intermediate Representation) constraints
- Generating execution traces as tabular data
- Configuring STARK components (fields, hash functions, commitments)
- Creating and verifying STARK proofs

### Key Concepts

- **Single-row constraints**: Each row is validated independently
- **Stateless computation**: No dependencies between rows
- **Basic AIR structure**: Foundation for all Plonky3 circuits

### Quick Start

```bash
cd plonky3-simple-circuit-implementation
cargo build --release
RUSTFLAGS="-Ctarget-cpu=native" cargo run --release
```

### Expected Output

```
🧮 Plonky3 Arithmetic Proof System
   Proving: a + c * d = e
   Values: 3 + 4 * 5 = 23

✅ Generated execution trace:
   256 rows: [a=3, c=4, d=5, e=23] (repeated)
   Constraint: a + c * d - e = 0
   Check: 3 + 4 * 5 - 23 = 0 ✅

🔄 Generating STARK proof...
✅ Proof generated successfully!

🔍 Verifying proof...
🎉 Proof verified successfully!
```

### Code Structure

```
plonky3-simple-circuit-implementation/
├── src/
│   └── main.rs          # Complete implementation
├── Cargo.toml           # Dependencies
└── README.md
```

**[📖 Full Documentation](https://www.notion.so/your-arithmetic-guide-link)**

---

## 🔢 Example 2: Fibonacci Sequence

**Proves:** `F(n) = F(n-1) + F(n-2)`

### What You'll Learn

- Multi-row constraint definition (referencing current AND next rows)
- State transition management in execution traces
- Sequential computation patterns
- How Plonky3 naturally handles stateful computations

### Why Fibonacci Matters

Unlike the arithmetic example, Fibonacci demonstrates **state transitions** - each step depends on previous steps. This pattern is essential for:

- zkVMs (virtual machines)
- Blockchain state transitions  
- Iterative algorithms
- Any sequential computation

### Key Differences from Arithmetic

| Aspect | Arithmetic Example | Fibonacci Example |
|--------|-------------------|-------------------|
| **Dependencies** | No row dependencies | Each row depends on previous |
| **State** | Stateless | Stateful |
| **Constraints** | Single-row: `a + c*d - e = 0` | Multi-row: `next.b = local.a + local.b` |
| **Demonstrates** | Basic AIR structure | State machines & transitions |

### Quick Start

```bash
cd plonky3-fibonacci-guide-Understanding-state-transitions
cargo build --release
RUSTFLAGS="-Ctarget-cpu=native" cargo run --release
```

### Expected Output

```
🔢 Plonky3 Fibonacci Proof System
   Proving: F(n) = F(n-1) + F(n-2)
   Computing: F(0)=0, F(1)=1, F(2)=1, F(3)=2, F(4)=3, F(5)=5...

✅ Generated execution trace:
   Computing 100 Fibonacci numbers
   Trace padded to 128 rows (power of 2)

   Sample values:
   F(0) = 1
   F(5) = 8
   F(10) = 89
   F(20) = 10946
   F(50) = 20365011074

   Constraints:
   1. Transition: next.b = local.a + local.b (Fibonacci rule)
   2. Propagation: next.a = local.b (state shift)

🔄 Generating STARK proof...
✅ Proof generated successfully!

🔍 Verifying proof...
🎉 Proof verified successfully!
```

### Code Structure

```
plonky3-fibonacci-guide-Understanding-state-transitions/
├── src/
│   └── main.rs          # Complete Fibonacci implementation
├── Cargo.toml           # Dependencies
└── README.md
```

**[📖 Full Documentation](https://www.notion.so/your-fibonacci-guide-link)**

---

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.70 or later
- **Cargo** (comes with Rust)

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone the Repository

```bash
git clone https://github.com/yourusername/plonky3-examples.git
cd plonky3-examples
```

### Run Examples

Each example is self-contained in its own directory:

```bash
# Run arithmetic example
cd plonky3-simple-circuit-implementation
cargo run --release

# Run fibonacci example  
cd ../plonky3-fibonacci-guide-Understanding-state-transitions
cargo run --release
```

### Build with Optimizations

For best performance:

```bash
RUSTFLAGS="-Ctarget-cpu=native" cargo run --release
```

---

## 📖 Understanding Plonky3

### What is Plonky3?

Plonky3 is Polygon's modular toolkit for building STARK-based zero-knowledge proof systems. Unlike its predecessor Plonky2, it uses:

- **AIR (Algebraic Intermediate Representation)** instead of gate-based circuits
- **Modular components** that can be mixed and matched
- **STARK proofs** optimized for large computations

### Key Architectural Shift: Plonky2 → Plonky3

| Feature | Plonky2 | Plonky3 |
|---------|---------|---------|
| **Approach** | Gate-based (CircuitBuilder) | Constraint-based (AirBuilder) |
| **Circuit Building** | Connect gates like electronic circuits | Define polynomial equations |
| **Scalability** | Fixed circuit size | Dynamic trace length |
| **Backend** | PLONK SNARKs | STARK proofs |
| **Use Case** | Complete proving system | Modular toolkit |

### Why This Matters

**Plonky2 CircuitBuilder:**
```rust
// Unroll computation at circuit build time
for i in 0..1000 {
    f_next = builder.add(f_prev, f_curr);
    // Creates 1000 gates
}
```

**Plonky3 AirBuilder:**
```rust
// Define constraint once, apply to all rows
builder.assert_zero(next.b - local.a - local.b);
// Single constraint for any number of steps
```

---

## 🔧 Project Structure

```
plonky3-examples/
├── plonky3-simple-circuit-implementation/
│   ├── src/
│   │   └── main.rs                    # Arithmetic implementation
│   ├── Cargo.toml
│   └── README.md
│
├── plonky3-fibonacci-guide-Understanding-state-transitions/
│   ├── src/
│   │   └── main.rs                    # Fibonacci implementation
│   ├── Cargo.toml
│   └── README.md
│
├── README.md                          # This file
└── LICENSE
```

---

## 🎓 What You'll Learn

### From the Arithmetic Example

✅ Basic AIR constraint definition  
✅ Execution trace generation  
✅ STARK configuration (fields, hash functions, commitments)  
✅ Proof generation and verification workflow  
✅ How Plonky3's modular architecture works  

### From the Fibonacci Example

✅ Multi-row constraints and state transitions  
✅ Sequential computation patterns  
✅ Why Plonky3 scales better than gate-based approaches  
✅ State machine design in ZK proofs  
✅ Dynamic trace generation at runtime  

---

## 📚 Additional Resources

### Official Documentation

- [Plonky3 GitHub Repository](https://github.com/Plonky3/Plonky3)
- [Polygon Knowledge Layer - Plonky3 Docs](https://docs.polygon.technology/)
- [Plonky3 Production Announcement](https://polygon.technology/blog/polygon-plonky3-is-production-ready)

### Our Guides

- [Complete Plonky3 Beginner's Guide](https://www.notion.so/your-link) - Comprehensive introduction
- [AIR vs CircuitBuilder Comparison](https://www.notion.so/your-link) - Architectural deep dive
- [Fibonacci State Transitions Guide](https://www.notion.so/your-link) - Advanced patterns

### Community

- [Polygon Discord](https://discord.gg/polygon)
- [Plonky3 Discussions](https://github.com/Plonky3/Plonky3/discussions)

---

## 🤝 Contributing

Contributions are welcome! If you'd like to add more examples or improve existing ones:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-example`)
3. Commit your changes (`git commit -m 'Add new example'`)
4. Push to the branch (`git push origin feature/new-example`)
5. Open a Pull Request

### Ideas for Future Examples

- Merkle tree proof verification
- Hash chain validation
- Simple VM instruction execution
- Range proofs
- Custom field arithmetic

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Polygon Labs** for developing Plonky3
- **Plonky3 contributors** for the excellent toolkit
- The **ZK community** for knowledge sharing and support

---

## 📧 Contact & Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/plonky3-examples/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/plonky3-examples/discussions)
- **Twitter**: [@yourhandle](https://twitter.com/yourhandle)

---

## 🗺️ Roadmap

- [x] Simple arithmetic circuit example
- [x] Fibonacci sequence with state transitions
- [ ] Merkle tree verification example
- [ ] Hash chain validation
- [ ] Simple VM example
- [ ] Range proof implementation
- [ ] Performance benchmarks
- [ ] Interactive tutorial notebooks

---

**Start learning Plonky3 today! Begin with the [Simple Arithmetic Circuit](./plonky3-simple-circuit-implementation) →**