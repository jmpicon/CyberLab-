# CyberLab: The Ultimate Cybersecurity Simulator

[![Status](https://img.shields.io/badge/Status-Playable-brightgreen)](#)
[![Security](https://img.shields.io/badge/Sandbox-Verified-blue)](#)

CyberLab is a premium cybersecurity simulator for PC featuring a **real Linux terminal** running inside an isolated Docker sandbox.

## 🚀 Key Features
- **Real Terminal**: Commands like `ls`, `grep`, `nmap`, and `cat` are executed in a real PTY environment.
- **Isolated Sandbox**: No internet access or LAN access from labs. Your host is 100% safe.
- **30+ Missions**: From Linux fundamentals to advanced Web exploitation.
- **Career Progression**: Earn credits, upgrade your rig, and unlock new mission packs.
- **Noir Vibe**: Immersive 3D environment built in Unreal Engine 5.

## 🛠️ Requirements
- **OS**: Windows (Docker Desktop + WSL2) or Linux (Docker native).
- **Docker**: Must be installed and running.
- **Hardware**: 8GB RAM minimum, Core i5/Ryzen 5+.

## 🕹️ How to Start
1.  **Backend**: `cd backend && cargo run`
2.  **Client**: Launch the `CyberLab.exe` (or run from UE5 Editor).
3.  **Terminal**: Type `help` for internal commands or start a mission from the dashboard.

## 📖 Mission Categories
- **Linux Fundamentals**: File permissions, process management, logs.
- **Networking**: Recon, sniffing, internal routing.
- **Blue Team**: SOC, hardening, incident response.
- **Forensics**: Log analysis, file recovery.
- **Web Security**: Controlled exploitation of vulnerable local apps.

---
*Disclaimer: This game is for educational purposes. All techniques are demonstrated within a controlled sandbox.*
