# GDD: CyberLab - The Ultimate Cybersecurity Simulator

## 1. Vision
**CyberLab** is a high-fidelity cybersecurity simulation game where players assume the role of an aspiring cyber-professional. Unlike other simulators, CyberLab features a **real** Linux terminal environment, forcing players to learn actual commands, networking, and security concepts in a safe, isolated sandbox.

## 2. Core Gameplay Loop
1.  **Selection**: Accept a mission from the "NetWire" dashboard.
2.  **Environment Setup**: The game spins up a custom Docker environment specific to the mission.
3.  **Execution**: Use the real internal terminal to accomplish objectives (e.g., gain access, recover logs, harden a server).
4.  **Completion**: System validates the state of the sandbox.
5.  **Reward**: Earn Credits ($) and Reputation (Rep).
6.  **Progression**: Upgrade your real-world hardware, move to better apartments, and unlock higher-tier mission packs.

## 3. Key Pillars
*   **Total Realism**: Real PTY, real bash/zsh, real filesystems.
*   **Hard Sandbox**: No internet, no host access, strictly isolated mission networks.
*   **Narrative Vibe**: Gritty, noir-cyberpunk aesthetic. Rain, low-fi beats, glowing CRTs.
*   **Pedagogy**: Learn by doing. Built-in hints and a "Learning Mode" for beginners.

## 4. Narrative & Paths
*   **White Hat**: Legal contracts, corporate auditing, incident response.
*   **Grey Hat**: High-stakes, morally ambiguous tasks.
*   **Black Hat**: Illegal (in-game) activities, high risk of getting caught by the "Cyber-Police" (NPC logic).

## 5. Technical Stack
*   **Grand Client**: Unreal Engine 5. Focus on UI shaders and immersive 3D/2D hybrid visuals.
*   **The Brain (Backend)**: Rust-based local orchestrator. Manages Docker, PTY streams, and mission state.
*   **Missions**: Data-driven YAML files defining the labs, objectives, and validation scripts.

## 6. Security Manifest (Hard Rules)
1.  `docker-compose` files must use custom networks with `internal: true`.
2.  Iptables on the host (if needed) to block all traffic from Docker subnets to the internet/LAN.
3.  No volume mounts of host sensitive directories.
4.  Mandatory timeout for all labs.
