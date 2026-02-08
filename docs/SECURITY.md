# Security Manifest: CyberLab Isolation Protocols

This document outlines the security measures taken to ensure that the cybersecurity labs remain strictly isolated sandboxes, preventing any accidental or intentional misuse for real-world attacks.

## 1. Network Isolation
*   **Disabled External Access**: Mission containers are connected to a Docker network with the `internal: true` flag. This prevents traffic from leaving the Docker subnet to the host's physical network or the Internet.
*   **Iptables Rules (Fail-safe)**: The backend orchestrator can optionally apply host-level iptables rules to drop all traffic originating from the specific mission bridge IDs.
*   **No Bridge to Host**: Containers never use `network_mode: host`.

## 2. Filesystem Protection
*   **No Sensitive Mounts**: The orchestrator is prohibited from mounting any host directory except the temporary workspace assigned to the mission.
*   **Read-Only Root**: Where possible, containers are started with a read-only root filesystem, with specific `tmpfs` mounts for `/tmp`, `/var/run`, etc.
*   **Ephemeral Workspace**: All files created during a mission are wiped immediately upon mission termination.

## 3. Resource Constraints (DoS Prevention)
*   **CGroup Limits**: Each mission container has hard limits on:
    *   CPU (e.g., max 1 core)
    *   Memory (e.g., max 512MB)
    *   Pids (e.g., max 100 processes)
*   **Timeouts**: Missions have a maximum duration (e.g., 60 minutes) before the orchestrator force-kills the sandbox.

## 4. Operational Security (OPSEC) Gamification
*   Logs generated inside the sandbox are monitored by the `Validator`. In the game, leaving too many log traces or failing to clear history might result in "Detection" by NPCs, leading to mission failure or reputation loss.

## 5. Non-Aggression Principle
*   **Tools Only**: Tools included in the images (e.g., `nmap`, `hydra`, `metasploit-framework`) are older versions or restricted versions designed specifically for the lab environments.
*   **Tutorial Content**: Documentation focuses on the *logic* and *concepts* rather than providing "cut-and-paste" commands for real targets.
