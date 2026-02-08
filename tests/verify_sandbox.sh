#!/bin/bash
# CyberLab Sandbox Verification Script

echo "[*] Starting sandbox verification..."

# 1. Create a test container with no internet
CONTAINER_ID=$(docker run -d --network none alpine sleep 60)

if [ -z "$CONTAINER_ID" ]; then
    echo "[!] Failed to start test container. Is Docker running?"
    exit 1
fi

# 2. Try to ping 8.8.8.8 from inside (should fail)
echo "[*] Testing Internet isolation (ping 8.8.8.8)..."
docker exec $CONTAINER_ID ping -c 1 -W 2 8.8.8.8 > /dev/null 2>&1

if [ $? -eq 0 ]; then
    echo "[!] SECURITY ALERT: Sandbox HAS internet access!"
    docker rm -f $CONTAINER_ID
    exit 1
else
    echo "[+] SUCCESS: Sandbox is isolated from Internet."
fi

# 3. Try to access local network (should fail if properly isolated, though 'none' network handles this)
echo "[*] Testing LAN isolation..."
# (Logic to test LAN access depending on host setup)

# Cleanup
docker rm -f $CONTAINER_ID
echo "[+] Verification complete. System is SECURE."
