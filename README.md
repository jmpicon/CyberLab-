# CyberLab: The Ultimate Cybersecurity Simulator

[![Status](https://img.shields.io/badge/Status-Playable-brightgreen)](#)
[![Security](https://img.shields.io/badge/Sandbox-Verified-blue)](#)

CyberLab is a premium cybersecurity simulator for PC featuring a **real Linux terminal** running inside an isolated Docker sandbox.

## � Linux First
CyberLab is built for Linux. It uses native PTY and Docker acceleration to provide the most realistic terminal experience possible.

## �🚀 Key Features
- **Native Linux Terminal**: Commands like `ls`, `grep`, `nmap` run directly on a high-perf PTY.
- **Docker Sandbox**: Secure, isolated environments for every mission.
- **30+ Missions**: From fundamentals to advanced exploitation.
- **Noir Vibe**: Immersive UE5 environment.

## 🛠️ Quick Start (Linux)
```bash
chmod +x setup_linux.sh
./setup_linux.sh
```

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
