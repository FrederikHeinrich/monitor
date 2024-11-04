#!/bin/bash
CONFIG_PATH="/etc/monitor/config.toml"
DEFAULT_CONFIG_PATH="/etc/monitor/default_config.toml"

# Systemd-Daemon neu laden und den Service starten/aktivieren
systemctl daemon-reload
systemctl enable monitor.service
systemctl start monitor.service

# Konfigurationsdatei kopieren, falls sie noch nicht existiert
if [ ! -f "$CONFIG_PATH" ]; then
    cp "$DEFAULT_CONFIG_PATH" "$CONFIG_PATH"
fi
