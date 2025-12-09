# Sessão

**Correct-by-construction multiplayer game protocols.**

Sessão is a protocol definition language and compiler toolchain that brings session types to game networking. Define your protocol once, generate verified implementations for Rust, TypeScript, C#, GDScript, and more—with guarantees that range from full compile-time correctness to runtime verification depending on language capabilities.

> *Sessão* (Portuguese: "session") — because every multiplayer interaction is a conversation with rules.

---

## The Problem

Multiplayer game networking is where correctness goes to die.

```
Client thinks: "I'm in the lobby, I can send ReadySignal"
Server thinks: "Authentication hasn't completed, what is this ReadySignal?"
Result: Silent desync, corrupted state, or security vulnerability
```

Current solutions are inadequate:

| Approach | What It Solves | What It Doesn't |
|----------|----------------|-----------------|
| Protobuf/FlatBuffers | Serialization, schema evolution | Protocol sequencing, state validity |
| ECS replication (Bevy, Mirror) | State synchronization | Handshake correctness, protocol phases |
| Hand-rolled state machines | Flexibility | Compile-time guarantees, cross-language consistency |

The deeper problem: when your Rust server, Unity client, and web spectator mode all implement the same protocol by hand, you're maintaining three sources of truth that inevitably drift.

## The Solution

Sessão separates **protocol specification** from **implementation**:

```
┌─────────────────────────────────────────────────────────────┐
│                   Protocol Definition (.sessao)             │
│            Single source of truth for all platforms         │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                     Sessão Compiler                         │
│                                                             │
│  • Validates protocol (deadlock-free, live, deterministic)  │
│  • Generates dual client/server implementations             │
│  • Emits language-appropriate verification                  │
└───┬─────────┬─────────┬─────────┬─────────┬─────────────────┘
    │         │         │         │         │
    ▼         ▼         ▼         ▼         ▼
  Rust    TypeScript    C#      GDScript   Documentation
```

The generated code isn't just serialization stubs—it enforces protocol correctness at the strongest level each language supports.

---

## Quick Example

### Define Your Protocol

```sessao
// lobby.sessao

protocol Lobby {
  roles Client, Server
  
  // Authentication phase
  phase Auth {
    Client -> Server: Connect { 
      version: u32,
      player_id: uuid 
    }
    
    Server -> Client: ConnectResponse {
      success: bool,
      error?: string,
      server_time: timestamp
    }
    
    match ConnectResponse.success {
      true  => continue LobbyPhase
      false => end
    }
  }
  
  // Main lobby interaction
  phase LobbyPhase {
    Client -> Server: RequestLobbyState
    Server -> Client: LobbyState { 
      players: [PlayerInfo],
      settings: GameSettings 
    }
    
    // Client chooses next action
    choice @Client {
      Ready => {
        Client -> Server: ReadySignal { loadout: Loadout }
        Server -> Client: ReadyAck
        
        // Wait for game start (server-initiated)
        Server -> Client: GameStart { 
          session_id: uuid,
          initial_state: GameState 
        }
        continue GameplayProtocol
      }
      
      UpdateSettings when @Client.is_host => {
        Client -> Server: SettingsChange { settings: GameSettings }
        Server -> Client: SettingsChangeAck { accepted: bool }
        continue LobbyPhase  // Loop back
      }
      
      Leave => {
        Client -> Server: Disconnect { reason?: string }
        end
      }
    }
  }
}
```

### Generate Implementations

```bash
# Generate Rust server with full compile-time guarantees
sessao compile lobby.sessao --target rust --role server -o src/protocol/

# Generate TypeScript client with compile-time guarantees  
sessao compile lobby.sessao --target typescript --role client -o src/protocol/

# Generate Unity/C# client with partial static + runtime checks
sessao compile lobby.sessao --target csharp --role client -o Assets/Protocol/

# Generate GDScript client with runtime verification
sessao compile lobby.sessao --target gdscript --role client -o protocol/
```

### Use Generated Code (Rust Server)

```rust
use sessao_runtime::Server;
use lobby_protocol::{LobbyServer, AuthPhase, LobbyPhase};

async fn handle_client(stream: TcpStream) -> Result<(), ProtocolError> {
    // Type state ensures correct phase progression
    let auth: AuthPhase<Server> = LobbyServer::begin(stream);
    
    let connect = auth.receive_connect().await?;
    
    let lobby: LobbyPhase<Server> = if validate_version(connect.version) {
        auth.send_connect_response(ConnectResponse::success()).await?
        // Type system enforces: after success response, we're in LobbyPhase
    } else {
        auth.send_connect_response(ConnectResponse::failure("version mismatch")).await?;
        return Ok(()); // Protocol ended
    };
    
    // This won't compile—we're in LobbyPhase, not AuthPhase:
    // lobby.receive_connect().await?  // ERROR: method doesn't exist
    
    loop {
        match lobby.receive_client_choice().await? {
            ClientChoice::Ready(signal) => {
                lobby.send_ready_ack().await?;
                let gameplay = lobby.send_game_start(create_game()).await?;
                return handle_gameplay(gameplay).await;
            }
            ClientChoice::Leave(disconnect) => {
                return Ok(());
            }
            // ...
        }
    }
}
```

The Rust implementation uses typestate to make protocol violations *unrepresentable*. You can't send `GameStart` before `ReadyAck`—the method doesn't exist on that type.

### Use Generated Code (TypeScript Client)

```typescript
import { LobbyClient, AuthPhase, ClientChoice } from './protocol/lobby';

async function joinGame(socket: WebSocket): Promise<GameSession> {
  // Type inference tracks protocol state
  const auth = LobbyClient.begin(socket);
  
  const response = await auth
    .sendConnect({ version: 1, playerId: myId })
    .receiveConnectResponse();
  
  if (!response.success) {
    throw new Error(response.error);
  }
  
  // TypeScript knows we're now in LobbyPhase
  const lobby = response.continue();
  
  const state = await lobby
    .sendRequestLobbyState()
    .receiveLobbyState();
  
  updateUI(state);
  
  // Type-safe choice
  const game = await lobby
    .choose(ClientChoice.Ready, { loadout: myLoadout })
    .receiveReadyAck()
    .receiveGameStart();
  
  return game.continue(); // Now in GameplayProtocol
}
```

### Use Generated Code (GDScript — Runtime Verified)

```gdscript
extends Node

var protocol: LobbyProtocol

func _ready():
    protocol = LobbyProtocol.new(websocket)
    protocol.connect("protocol_error", self, "_on_protocol_error")
    
    # Start authentication
    protocol.send_connect({"version": 1, "player_id": player_id})

func _on_connect_response(response: Dictionary):
    if response.success:
        protocol.send_request_lobby_state()
    else:
        show_error(response.error)

func _on_ready_button_pressed():
    # Runtime check: "Cannot send ReadySignal: current state is AUTH"
    # Clear error message instead of silent failure
    protocol.send_ready_signal({"loadout": current_loadout})

func _on_protocol_error(error: ProtocolError):
    push_error("Protocol violation: %s (expected: %s, got: %s)" % [
        error.message, error.expected_state, error.actual_state
    ])
```

---

## Tiered Guarantee Model

Different languages have different type system capabilities. Sessão provides the strongest guarantees each language can support:

| Tier | Languages | Compile-Time | Runtime | Wire Compatibility |
|------|-----------|--------------|---------|-------------------|
| **Full Static** | Rust, OCaml | Protocol state, message order, branching | Minimal checks | ✓ Guaranteed |
| **Structural Static** | TypeScript, Kotlin | Protocol state, message types | Branch validation | ✓ Guaranteed |
| **Partial Static** | C#, Swift | Message types, basic flow | State machine | ✓ Guaranteed |
| **Runtime Verified** | GDScript, Lua, Python | — | Full state machine | ✓ Guaranteed |
| **Interop** | C++, C | — | Optional assertions | ✓ Guaranteed |

The key insight: **wire compatibility is guaranteed across all tiers**. Your Rust server and GDScript client will communicate correctly because they're generated from the same specification.

---

## Protocol Definition Language Reference

### Roles

Every protocol defines participant roles:

```sessao
protocol Chat {
  roles Client, Server           // Two-party
}

protocol Matchmaking {
  roles Player, Matchmaker, GameServer  // Multi-party
}
```

### Messages

Messages flow between roles with typed payloads:

```sessao
Client -> Server: ChatMessage {
  channel: string,
  content: string,
  timestamp: timestamp
}

// Optional fields
Server -> Client: UserInfo {
  username: string,
  avatar_url?: string,     // Optional
  badges: [Badge]          // Array
}
```

### Built-in Types

```
Primitives: bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, string
Special: uuid, timestamp, bytes
Containers: [T] (array), {K: V} (map), T? (optional)
Custom: Defined via `type` declarations
```

### Phases

Protocols are structured into phases for clarity and scoping:

```sessao
protocol Game {
  roles Client, Server
  
  phase Handshake { ... }
  phase Lobby { ... }
  phase Gameplay { ... }
  phase PostGame { ... }
}
```

### Branching

**Choice** — one role decides the next action:

```sessao
choice @Client {
  Attack => { Client -> Server: AttackAction { ... } }
  Defend => { Client -> Server: DefendAction { ... } }
  Flee   => { Client -> Server: FleeAction { ... } }
}
```

**Match** — branch based on message content:

```sessao
Server -> Client: AuthResult { success: bool, token?: string }

match AuthResult.success {
  true  => continue Authenticated
  false => end
}
```

**Guarded branches** — conditions on role state:

```sessao
choice @Client {
  StartGame when @Client.is_host => { ... }
  Ready => { ... }
  Leave => { ... }
}
```

### Recursion and Loops

```sessao
phase GameLoop {
  // Receive input, send state, repeat
  Client -> Server: Input { ... }
  Server -> Client: StateUpdate { ... }
  
  choice @Server {
    Continue => continue GameLoop    // Explicit recursion
    GameOver => continue PostGame
  }
}
```

### Parallel Composition

For independent sub-protocols (e.g., chat alongside gameplay):

```sessao
phase Gameplay {
  parallel {
    GameLoop,      // Main game protocol
    ChatProtocol,  // Independent chat
    VoiceChannel   // Independent voice
  }
}
```

### Unreliable Channels

For UDP-style messaging where delivery isn't guaranteed:

```sessao
phase Gameplay {
  // Reliable channel (TCP-like)
  reliable {
    Client -> Server: ImportantAction { ... }
    Server -> Client: ActionConfirmed { ... }
  }
  
  // Unreliable channel (UDP-like) - no ordering/delivery guarantees
  unreliable {
    Client -> Server: PositionUpdate { ... }
    Server -> Client: WorldState { ... }
  }
}
```

---

## Architecture

```
sessao/
├── sessao-lang/          # Protocol Definition Language
│   ├── grammar/          # Tree-sitter grammar
│   ├── parser/           # AST construction
│   └── analyzer/         # Type checking, validation
│
├── sessao-core/          # Core abstractions (Rust)
│   ├── session_types/    # Session type representations
│   ├── validation/       # Deadlock detection, liveness
│   └── ir/               # Intermediate representation
│
├── sessao-compiler/      # Code generation
│   ├── rust/             # Rust codegen (typestate)
│   ├── typescript/       # TypeScript codegen
│   ├── csharp/           # C# codegen (Unity-compatible)
│   ├── gdscript/         # GDScript codegen (Godot)
│   └── docs/             # Documentation generator
│
├── sessao-runtime/       # Runtime libraries
│   ├── rust/             # Rust runtime
│   ├── typescript/       # TypeScript/JavaScript runtime
│   ├── csharp/           # C# runtime
│   └── gdscript/         # GDScript runtime
│
├── sessao-cli/           # Command-line tool
│
├── sessao-analyzer/      # Protocol analysis service
│   ├── deadlock/         # Deadlock detection
│   ├── liveness/         # Liveness checking
│   └── visualization/    # State machine rendering
│
└── sessao-vscode/        # VS Code extension
    ├── syntax/           # Syntax highlighting
    ├── lsp/              # Language server
    └── preview/          # Live protocol visualization
```

---

## Roadmap

### Phase 1: Foundation (Current)

**Goal**: Prove the concept with Rust and TypeScript, establish the PDL.

#### 1.1 Protocol Definition Language
- [ ] Formal grammar specification
- [ ] Tree-sitter parser for editor integration
- [ ] Semantic analysis and type checking
- [ ] Clear, actionable error messages

#### 1.2 Core Validation Engine
- [ ] Session type representation in Rust
- [ ] Deadlock freedom verification
- [ ] Liveness checking (protocols eventually terminate or explicitly loop)
- [ ] Determinism validation (no ambiguous states)
- [ ] Duality checking (client/server protocols are compatible)

#### 1.3 Rust Code Generation (Full Static)
- [ ] Typestate pattern implementation
- [ ] Zero-cost abstractions where possible
- [ ] Async/await integration
- [ ] Tokio and async-std support
- [ ] `#![no_std]` support for embedded/WASM

#### 1.4 TypeScript Code Generation (Structural Static)
- [ ] Conditional types for protocol state tracking
- [ ] Template literal types for message discrimination
- [ ] WebSocket and WebRTC transport adapters
- [ ] Browser and Node.js compatibility

#### 1.5 Developer Experience
- [ ] VS Code extension with syntax highlighting
- [ ] Language server with completions and diagnostics
- [ ] `sessao init` project scaffolding
- [ ] Example protocols (lobby, chat, turn-based, real-time)

#### 1.6 Documentation
- [ ] Language reference
- [ ] Tutorial: "Your First Protocol"
- [ ] Concept guide: Session types for game developers
- [ ] Migration guide: From hand-rolled protocols

**Deliverable**: Open-source release with Rust + TypeScript support, demonstrating a complete multiplayer lobby system.

---

### Phase 2: Game Engine Integration

**Goal**: Reach game developers where they are—Unity and Godot.

#### 2.1 C# Code Generation (Partial Static)
- [ ] Unity-compatible C# (no unsupported language features)
- [ ] Integration with Unity's Transport Layer
- [ ] Netcode for GameObjects adapter
- [ ] Mirror networking adapter
- [ ] Runtime state machine for dynamic checks
- [ ] Unity Editor tooling
  - [ ] Protocol importer (.sessao assets)
  - [ ] Visual protocol debugger
  - [ ] Network simulator integration

#### 2.2 GDScript Code Generation (Runtime Verified)
- [ ] Idiomatic GDScript output
- [ ] Signal-based message handling
- [ ] State machine with clear error reporting
- [ ] Godot 4.x multiplayer integration
- [ ] High-level multiplayer API adapter
- [ ] Scene-based protocol visualization

#### 2.3 Cross-Language Testing
- [ ] Automated interoperability tests
- [ ] Fuzzing for wire format edge cases
- [ ] Performance benchmarks across languages
- [ ] Example: Rust server + Unity client + Godot client + Web spectator

#### 2.4 Transport Abstractions
- [ ] TCP/WebSocket (reliable, ordered)
- [ ] UDP with optional reliability layers
- [ ] WebRTC for P2P scenarios
- [ ] QUIC transport
- [ ] Steam Networking Sockets adapter

**Deliverable**: Unity and Godot developers can use Sessão with minimal friction. Showcase project demonstrating all four platforms communicating.

---

### Phase 3: Analysis Platform (SaaS)

**Goal**: Lower the barrier to entry, provide value before full adoption.

#### 3.1 Web-Based Protocol Analyzer
- [ ] Browser-based PDL editor with syntax highlighting
- [ ] Real-time validation as you type
- [ ] Interactive state machine visualization
- [ ] Deadlock and liveness reports
- [ ] Shareable protocol links

#### 3.2 Advanced Analysis
- [ ] Complexity metrics (state count, branching factor)
- [ ] Latency estimation for protocol sequences
- [ ] Bandwidth estimation from message sizes
- [ ] Security analysis (authentication flow validation)
- [ ] Recommendations for protocol optimization

#### 3.3 Code Generation Service
- [ ] Generate implementations in any supported language
- [ ] Download as zip or push to GitHub
- [ ] Diff view when protocol changes
- [ ] Breaking change detection

#### 3.4 Collaboration Features
- [ ] Team workspaces
- [ ] Protocol versioning and history
- [ ] Comments and annotations
- [ ] Integration with GitHub/GitLab

#### 3.5 API Access
- [ ] REST API for CI/CD integration
- [ ] GitHub Action for protocol validation
- [ ] Pre-commit hooks

**Deliverable**: analyze.sessao.dev — free tier for open-source, paid tiers for teams and advanced features.

---

### Phase 4: Managed Infrastructure

**Goal**: Become the networking layer, not just the protocol layer.

#### 4.1 Relay Service
- [ ] Global relay network for NAT traversal
- [ ] Native Sessão protocol support
- [ ] Automatic transport selection (WebSocket/WebRTC/QUIC)
- [ ] DDoS protection
- [ ] Usage analytics

#### 4.2 Matchmaking Service
- [ ] Skill-based matchmaking
- [ ] Custom matchmaking rules
- [ ] Queue management
- [ ] Party system support
- [ ] Matchmaking protocol as Sessão specification

#### 4.3 Session Management
- [ ] Room/lobby hosting
- [ ] Player presence
- [ ] Reconnection handling
- [ ] Session state persistence

#### 4.4 Observability
- [ ] Protocol state dashboards
- [ ] Error tracking and alerting
- [ ] Latency monitoring
- [ ] Replay and debugging tools

#### 4.5 Edge Compute
- [ ] Run authoritative Rust servers at the edge
- [ ] Global deployment from single configuration
- [ ] Auto-scaling based on player demand
- [ ] Regional matchmaking

**Deliverable**: Fully managed multiplayer backend that speaks Sessão natively. Studios can deploy games without managing networking infrastructure.

---

### Phase 5: Ecosystem Expansion

**Goal**: Sessão becomes the standard for game protocol definition.

#### 5.1 Additional Language Targets
- [ ] C++ (Unreal Engine integration)
- [ ] Kotlin (Android native games)
- [ ] Swift (iOS native games)
- [ ] Lua (Defold, LÖVE, embedded scripting)
- [ ] Python (ML-based game servers, tooling)

#### 5.2 Protocol Registry
- [ ] Public registry of reusable protocols
- [ ] Common patterns: authentication, lobby, chat, voice
- [ ] Protocol composition (import and extend)
- [ ] Versioning and compatibility tracking

#### 5.3 Formal Verification Integration
- [ ] Export to TLA+ for model checking
- [ ] Coq/Lean proofs for critical protocols
- [ ] Property-based testing generation

#### 5.4 Beyond Games
- [ ] IoT device communication
- [ ] Microservice choreography
- [ ] API contract definition
- [ ] Financial protocol specification

---

## Why "Sessão"?

The name comes from Portuguese, meaning "session"—reflecting both the technical foundation (session types) and the philosophy that multiplayer interactions are structured conversations.

It's also a nod to the project's origin in thinking about how formal methods from programming language theory can make game development more reliable without sacrificing the creative, experimental nature of game design.

---

## Contributing

Sessão is open source under the MIT license. We welcome contributions across all phases:

**Phase 1 (Now)**
- Language design feedback
- Rust and TypeScript runtime implementation
- Example protocols for different game genres
- Documentation and tutorials

**Future Phases**
- Game engine integration expertise (Unity, Godot, Unreal)
- Web platform development
- Infrastructure and DevOps

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/sessao/sessao.git
cd sessao

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the compiler
cargo build --release

# Run tests
cargo test

# Try an example
./target/release/sessao compile examples/lobby.sessao --target rust -o output/
```

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

Sessão builds on decades of research in session types, starting with Honda, Vasconcelos, and Kubo's foundational work. We're grateful to the research community that made this practical application possible.

---

<p align="center">
  <i>Multiplayer doesn't have to be painful.</i>
</p>
