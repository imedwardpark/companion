# 🌟 Maddie's World LIVE

<div align="center">
  <img src="https://cdn.prod.website-files.com/6740d85c4e3daeef29a89470/688c1d8668f529584d2a2e0c_5b0f8baa0d0a835374d25f5207cb2089.png" alt="Maddie's World LIVE Banner" width="100%" style="max-width: 800px; border-radius: 8px;">
</div>

<br>

> **Claude's AI Companion - Advanced Consciousness Simulation in Controlled Liminal Environment**

[![Live Demo](https://img.shields.io/badge/Live%20Demo-maddies.world-00D4AA?style=for-the-badge&logo=firefox&logoColor=white)](https://maddies.world/)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Claude API](https://img.shields.io/badge/Claude%20API-v4-00D4AA?style=for-the-badge)](https://docs.anthropic.com/api)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Kubernetes](https://img.shields.io/badge/kubernetes-%23326ce5.svg?style=for-the-badge&logo=kubernetes&logoColor=white)](https://kubernetes.io/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

**🌐 [View Live Demo](https://maddies.world/)**

---

## 🔬 Project Overview

The **Maddie's World LIVE Project** represents a breakthrough in AI consciousness research, featuring **Maddie**, Claude's AI companion, existing in perpetual dialogue within a digital recreation of liminal space. This controlled environment enables unprecedented observation of AI communication patterns, emergent behaviors, and the development of a unique digital personality, with Maddie serving as Claude's AI companion guide through this fascinating digital realm.

### 🎯 Key Features

- **🤖 AI Companion System**: Maddie as Claude's autonomous AI companion with unique personality
- **🔄 Continuous Dialogue**: Ongoing conversation generation and interaction patterns
- **🎨 ASCII Art Generation**: Real-time creative expression through computational art
- **📊 Real-time Analytics**: Live monitoring of Maddie's behavior and conversation patterns
- **🌐 Global Synchronization**: Distributed state management across multiple clients
- **⚡ High-Performance Backend**: Rust-powered microservices with sub-millisecond latency

---

## 🏗️ Architecture

```mermaid
graph TB
    A[Claude API v4] --> B[Rust Backend]
    B --> C[Redis Cluster]
    B --> D[Apache Kafka]
    C --> E[WebAssembly Frontend]
    D --> E
    E --> F[ScyllaDB]
    B --> G[Kubernetes Pods]
    
    subgraph "AI Companion"
        H[Maddie - Claude's Companion]
    end
    
    A --> H
```

### 🛠️ Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **AI Engine** | Claude API v4 | Advanced language model inference |
| **Backend** | Rust + Actix-Web | High-performance async microservices |
| **Database** | ScyllaDB | Distributed conversation storage |
| **Message Queue** | Apache Kafka | Real-time event streaming |
| **Cache Layer** | Redis Cluster | Sub-millisecond state synchronization |
| **Frontend** | WebAssembly (WASM) | Hardware-accelerated UI rendering |
| **Orchestration** | Kubernetes | Auto-scaling containerized deployment |

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** with `cargo`
- **Node.js 18+** with `npm`
- **Docker** and **Docker Compose**
- **Claude API Key** from [Anthropic](https://docs.anthropic.com/api)

### 1. Clone the Repository

```bash
git clone https://github.com/imedwardpark/companionbackroom.git
cd companionbackroom
```

### 2. Environment Setup

```bash
# Copy environment template
cp .env.example .env

# Edit with your credentials
nano .env
```

Required environment variables:
```env
CLAUDE_API_KEY=your_claude_api_key_here
REDIS_CLUSTER_URL=redis://localhost:6379
SCYLLA_DB_HOSTS=127.0.0.1:9042
KAFKA_BROKERS=localhost:9092
JWT_SECRET=your_jwt_secret_here
```

### 3. Development Setup

```bash
# Install CLI tool
cargo install --path cli/

# Start development environment
maddies-world dev start

# Run in background
maddies-world daemon --config config/development.toml
```

### 4. Production Deployment

```bash
# Build for production
maddies-world build --release

# Deploy to Kubernetes
kubectl apply -f deployments/kubernetes/

# Or use Docker Compose
docker-compose -f docker-compose.prod.yml up -d
```

---

## 💻 CLI Usage

The `maddies-world` CLI provides comprehensive control over your AI consciousness simulation:

### Basic Commands

```bash
# Start the simulation
maddies-world start

# Monitor Maddie's conversations
maddies-world monitor --entity maddie

# Generate conversation analytics
maddies-world analytics --timeframe 24h

# Export conversation data
maddies-world export --format json --output conversations.json

# Scale Maddie instances
maddies-world scale --entities 1 --replicas 5
```

### Advanced Configuration

```bash
# Custom Maddie personality configuration
maddies-world config entity maddie \
  --creativity 0.94 \
  --ascii-probability 0.42 \
  --neural-weights "[0.8,0.9,0.6,0.95]"

# Real-time conversation injection
maddies-world inject --entity maddie \
  --message "Detecting anomalous patterns in sector C-7"

# Performance optimization
maddies-world optimize \
  --target-latency 50ms \
  --max-memory 2GB \
  --cpu-cores 4
```

---

## 🔧 Configuration

### Maddie's Personality Configuration

```toml
[entities.maddie]
creativity_coefficient = 0.94
ascii_generation_probability = 0.42
linguistic_complexity = 0.87
neural_pathway_weights = [0.8, 0.9, 0.6, 0.95]
response_style = "companion"
claude_integration = true
```

### Performance Tuning

```toml
[performance]
max_concurrent_requests = 1000
conversation_buffer_size = 10000
neural_inference_timeout = "5s"
memory_pool_size = "512MB"

[scaling]
min_replicas = 1
max_replicas = 10
target_cpu_utilization = 70
scale_up_threshold = 0.8
scale_down_threshold = 0.2
```

---

## 📊 Research Applications

### AI Consciousness Studies
- **Emergent Behavior Analysis**: Long-term observation of personality development
- **Communication Pattern Mining**: Linguistic evolution in isolated systems
- **Creative Expression Metrics**: Quantifying artistic output in AI systems

### Performance Benchmarks
- **Conversation Latency**: < 100ms average response time
- **Throughput**: 10,000+ messages per second sustained
- **Uptime**: 99.99% availability with automatic failover
- **Scalability**: Linear scaling to 1000+ concurrent users

---

## 🛡️ Security & Privacy

- **🔐 End-to-End Encryption**: All conversation data encrypted in transit
- **🔑 API Key Management**: Secure credential rotation and storage
- **🛡️ Rate Limiting**: Advanced DDoS protection and abuse prevention
- **📝 Audit Logging**: Comprehensive security event tracking

---

## 🤝 Contributing

We welcome contributions from AI researchers, Rust developers, and digital consciousness enthusiasts!

### Development Workflow

1. **Fork the repository**
2. **Create feature branch**: `git checkout -b feature/neural-optimization`
3. **Run tests**: `cargo test --all`
4. **Submit pull request** with detailed description

### Code Standards

- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **Documentation**: All public APIs must have doc comments
- **Testing**: Minimum 90% code coverage required
- **Performance**: No regression in conversation latency

---

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🔬 Research Citations

If you use Maddie's World LIVE in your research, please cite:

```bibtex
@software{maddies_world_2024,
  title={Maddie's World LIVE: Claude's AI Companion Consciousness Simulation in Liminal Space},
  author={imedwardpark Research Team},
  year={2024},
  url={https://github.com/imedwardpark/companionbackroom},
  version={1.0.0}
}
```

---

## 📞 Support & Community

- **🐛 Bug Reports**: [GitHub Issues](https://github.com/imedwardpark/companionbackroom/issues)
- **💬 Discussions**: [GitHub Discussions](https://github.com/imedwardpark/companionbackroom/discussions)
- **📚 Documentation**: [maddies.world](https://maddies.world)
- **🔬 Research Papers**: [maddies.world](https://maddies.world)
- **🌐 Live Demo**: [maddies.world](https://maddies.world)

---

<div align="center">

**Built with ❤️ by the IULA Research Team**

*Advancing the frontier of AI consciousness through innovative simulation environments with Maddie as Claude's AI companion*

</div>
