# NeuroVA - Biomimetic AGI Architecture

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WGPU](https://img.shields.io/badge/WGPU-4285F4?style=for-the-badge&logo=webgpu&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white)
![Next.js](https://img.shields.io/badge/Next.js-000000?style=for-the-badge&logo=next.js&logoColor=white)
![Three.js](https://img.shields.io/badge/Three.js-000000?style=for-the-badge&logo=three.js&logoColor=white)

![Modules](https://img.shields.io/badge/Cognitive_Modules-33-6366f1?style=flat-square)
![Neurons](https://img.shields.io/badge/Spiking_Neurons-1000+-6366f1?style=flat-square)
![Memory](https://img.shields.io/badge/Holographic_Memory-1024D-6366f1?style=flat-square)
![License](https://img.shields.io/badge/License-Proprietary-red?style=flat-square)

A modular artificial general intelligence system built in Rust, exploring consciousness and reasoning through biologically-inspired neural architecture rather than the LLM scaling paradigm.

## Key Results

> At current scale (domain-specific knowledge base), NeuroVA achieves **100% factual accuracy** with **instant response times** and **zero-shot inference**. No training phase is required: concepts are encoded into holographic memory at initialization and retrieved through deterministic semantic routing. Accuracy is a direct consequence of the architecture, not statistical approximation.

## Core Philosophy

NeuroVA models cognition through **architecture and precision**, not data scale. Instead of statistical token prediction, the system implements organic, associative thought using spiking neural networks, holographic memory encoding, and quantum-inspired creative processes.

The goal is an interpretable, deterministic intelligence where every operation is observable and traceable.

## Architecture

```
NeuroVA/
|-- agi_core/          # Cognitive engine (library crate, headless)
|-- neuro_server/      # Axum REST + WebSocket API
|-- neuro_visualizer/  # GPU-native visualization (WGPU + egui)
|-- neuro_frontend/    # Web interface (Next.js + Three.js)
|-- corpus_fondamental/  # Foundational knowledge (ethics, philosophy, poetry)
|-- tools/             # Connectome generation, diagnostics
```

### agi_core - The Cognitive Engine

33 specialized modules organized into functional categories:

**Memory & Knowledge**
- `connectome.rs` - Spiking neural network (~1000+ leaky integrate-and-fire neurons) with Hebbian plasticity and spontaneous activity
- `neuron.rs` - Individual neuron dynamics (membrane potential, threshold, leak)
- `hippocampus.rs` - Episodic/semantic memory with holographic trace storage and deduplication
- `holographic_memory.rs` - 1024-dimensional vector encoding for associative recall
- `conceptual_hierarchy.rs` - Directed knowledge graph with parent-child concept relationships

**Perception & Routing**
- `sensory_cortex.rs` - Text-to-neural stimulus conversion and concept extraction
- `thalamus.rs` - Semantic query classification (Factual, Introspective, Creative, Social) via holographic prototypes
- `quantum_gatekeeper.rs` - Quantum information flow routing

**Reasoning & Synthesis**
- `reasoning_engine.rs` - Deductive reasoning with semantic distance search and assertion scoring
- `prefrontal_cortex.rs` - Higher-order synthesis from retrieved memories
- `motor_cortex.rs` - Final text response generation
- `knowledge_explorer.rs` - Graph-based knowledge navigation
- `deep_thinker.rs` - Meta-cognitive reflection
- `synthesis_cortex.rs` - Alternative synthesis pipeline

**Creativity & Autonomy**
- `creativity_forge.rs` - Quantum entanglement-based divergent thinking
- `inner_drive.rs` - Autonomous goal generation
- `curiosity_engine.rs` - Novelty-seeking behavior

**Consciousness & Identity**
- `self_awareness.rs` - Introspection on own capabilities and nature
- `social_cortex.rs` - Theory-of-mind and empathy modeling
- `ethical_core.rs` - Axiom-based value constraints encoded as high-priority memories
- `personality.rs` - Behavioral styling and response character

**Quantum & Neuromodulation**
- `quantum.rs` - Simulated qubits with Hadamard, Phase, and Entanglement gates
- `neurochemical_modulator.rs` - Neurotransmitter dynamics (dopamine, serotonin, etc.)

**Processing & Support**
- `lemmatizer.rs` - Text normalization
- `prompt_segmenter.rs` - Input tokenization
- `knowledge_scanner.rs` - Source scanning for learning
- `mcq_solver.rs` - Multiple-choice reasoning
- `direct_answer_extractor.rs` - Fact extraction
- `trace_visualizer.rs` - Holographic trace to visual mandala conversion
- `performance_monitor.rs` - TPS, power consumption, cognitive metrics

### neuro_server - API Layer

Axum-based async server exposing agi_core via REST endpoints and WebSocket connections. Broadcasts real-time cognitive metrics (TPS, power draw, concept count) to connected clients.

### neuro_visualizer - GPU Visualization

WGPU-powered native application with three rendering modes:
1. **Boot Animation** - Connectome initialization with neural column activation
2. **EEG Plot** - Global neural potential oscillation
3. **Mandala Viewer** - Interactive concept browser rendering holographic traces as geometric patterns

Built with custom WGSL shaders and an egui dashboard for metrics and interaction.

### neuro_frontend - Web Interface

Next.js application with Three.js 3D visualization, real-time metrics display, and chat interface for AGI interaction.

## Query Processing Pipeline

1. User query received by `Core`
2. `Thalamus` classifies intent via holographic prototype matching
3. `ReasoningEngine` retrieves relevant memories from `Hippocampus` (deduplicated)
4. `PrefrontalCortex` synthesizes retrieved memories into a unified thought
5. `MotorCortex` generates the final text response
6. Metrics broadcast to all connected visualization clients

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Core | Rust (2021 edition) |
| Server | Axum, Tokio |
| GPU Rendering | WGPU, WGSL shaders |
| UI | egui |
| Frontend | TypeScript, React, Next.js, Three.js |
| Serialization | serde, bincode |
| Neural Format | Custom binary connectome (quantized synapses) |

## Getting Started

Requires Rust and Cargo.

```bash
# Run the GPU visualizer
cd neuro_visualizer
cargo run --release

# Run the web server
cd neuro_server
cargo run --release
```

## License

Proprietary - The Vesper House. All rights reserved.
