# CyberLab Developer Guide

## Repository Structure
- `/client`: UE5 Project files (Source code in `/Source/CyberLab`).
- `/backend`: Rust components (Axum server, Orchestrator, PTY).
- `/missions`: YAML mission specifications and assets.
- `/tools`: Utilities (Validator, Creator).
- `/docs`: Detailed GDD, Architecture, and Security docs.

## Setup & Build
### Backend
```bash
cd backend
cargo build --release
./target/release/cyberlab-backend
```

### Unreal Client
1. Open `CyberLab.uproject` in UE5.4+.
2. Compile C++ project.
3. Press Play or Package for Win/Linux.

## Creating New Missions
1. Create a YAML file in `missions/<pack>/`.
2. Follow the [MISSION_SCHEMA.md](docs/MISSION_SCHEMA.md).
3. Validate using the tool:
```bash
python3 tools/mission_validator.py missions/my_pack
```

## Security Testing
Always run the sandbox verification script before a release:
```bash
./tests/verify_sandbox.sh
```
