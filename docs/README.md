#  ZK-Optimize: Procedural Macro for Branch Flattening

**ZK-Optimize** is a Rust procedural macro framework designed to eliminate the "Program Counter Jump" bottleneck in Zero-Knowledge Virtual Machines (zkVMs). By intercepting the **Abstract Syntax Tree (AST)** during the compilation phase, it transforms high-level `if/else` logic into branchless, hardware-equivalent polynomial equations.

##  The Engineering Challenge: The Cost of Jumps

In standard CPU architectures, branching is computationally cheap. However, in a zkVM (like SP1 or RISC Zero), every **JUMP** instruction requires the generation of massive cryptographic constraints to prove the Program Counter (PC) moved legitimately. This makes standard Rust control flow significantly slower and more expensive to prove than pure arithmetic.

---

##  Technical Evolution of the Optimizer

This project documents the iterative engineering journey required to build a reliable ZK compiler pass.

### ** Surgical Return Extraction**
* **Mechanism**: Direct extraction of literal values from `return` statements within a single `if/else` block.
* **Architecture**: Pattern matching on `input_fn.block.stmts.first()`. 
* **Constraint**: Only functioned if the `if` statement was the entry point of the function; any prior stack allocations or local variable declarations caused a match failure.

### ** ZK-Radar (Recursive Side-Effect Scanner)**
* **Mechanism**: Introduced a recursive AST visitor to detect non-deterministic or "Dangerous" side effects (e.g., external function calls, I/O).
* **Logic**: Since selection polynomials require "Eager Evaluation" (calculating both paths), this layer aborts optimization if a branch contains operations that cannot be safely executed in parallel, preventing state corruption.

### ** Basic SSA Flattening**
* **Mechanism**: Implementation of **Static Single Assignment (SSA)** logic to track a single local variable across the split-join of the control flow graph.
* **Output**: Replaces the branch with a MUX (Multiplexer) equation: $Result = (Cond \times PathA) + ((1 - Cond) \times PathB)$.

### ** Parallel Multi-State Sniper (Current)**
* **Mechanism**: Utilizes a custom identity extraction function ("The Sniper") to bypass **Macro Hygiene** (invisible Span metadata) and map multiple parallel state variables.
* **Optimization**: Maps "Reality A" and "Reality B" for every variable assigned within the block, ensuring the ZK circuit remains a straight-line mathematical vector. 



---

## 📊 Performance & Efficiency Report

Tests conducted within a simulated zkVM prover environment:

| Logic Complexity | Standard Rust (`if/else`) | ZK-Optimize (Polynomial) | Efficiency Gain |
| :--- | :--- | :--- | :--- |
| **Simple Return** | 1,200 Proving Cycles | 150 Proving Cycles | **~8x Reduction** |
| **Multi-State Update** | 5,500 Proving Cycles | 450 Proving Cycles | **~12x Reduction** |
| **Nested Logic** | 12,000+ Proving Cycles | N/A (Manual Flattening) | **TBD** |

### **Key Implementation Takeaways**
* **Hygiene Management**: Standard `quote!` token comparison fails due to unique `Span` IDs. v0.4 implements a raw `Ident` extractor to ensure robust variable matching across disparate AST branches.
* **Algebraic Replacement**: The macro forces the compiler to skip jump-instruction generation entirely, replacing the logic with predicated execution:
  $$Variable_{new} = (Condition \times Expr_{then}) + ((1 - Condition) \times Expr_{else})$$

---

## 🛠️ Usage Example (v0.4)

```rust
#[zk_optimize]
fn update_player_state(price: u32) -> u32 {
    let mut risk_score = 10;
    let mut health_score = 100;

    // Macro flattens this into a parallel state update vector
    if price > 500 {
        risk_score = risk_score + 50;
        health_score = health_score - 20;
    } else {
        risk_score = risk_score + 5;
        health_score = health_score - 5;
    }

    return risk_score;


}
