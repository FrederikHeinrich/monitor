#!/bin/bash
CONFIG_PATH="/etc/monitor/config.toml"

systemctl stop monitor.service
systemctl disable monitor.service
systemctl daemon-reload

# Konfigurationsdatei löschen, falls sie existiert
if [ -f "$CONFIG_PATH" ]; then
    rm "$CONFIG_PATH"
fi