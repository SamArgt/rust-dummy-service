# Rust Dummy Linux Service

To run a Rust program as a service, you'll typically use systemd on Linux systems. Here's a concise guide:

1. Build your Rust program for release:
```bash
cargo build --release
```

2. Create a systemd service file (e.g., `/etc/systemd/system/your-service.service`):
```ini
[Unit]
Description=Rust Dummy Service
After=network.target

[Service]
Type=simple
User=chmo
ExecStart=/home/chmo/rust-dummy-service/target/release/rust-dummy-service
# Restart=on-failure

[Install]
# WantedBy=multi-user.target
```

3. Enable and start the service:
```bash
sudo systemctl enable your-service
sudo systemctl start your-service
```

Key considerations:
- Use `Type=simple` for most services
- Set appropriate `User` and `WorkingDirectory`
- Use `Restart=on-failure` for resilience
- Check service status with `systemctl status your-service`

For more complex services, consider using tools like `systemfd` or `socket activation` for advanced process management.