# Marvin - Metrics Tracker
* Simple daemon built with Rust to track metrics.
* The goal is run application as a daemon and track specific metrics. 
* We also want to be able to send metrics to a server via a socket connection if option specified when starting the daemon.
* We would only send metrics to server if threasholds are met.
* Configuration and threasholds are specified in a config file.
* Initially only alerts would be sent, not every single measurement.

### Requirements
* [Rust](https://www.rust-lang.org/)
* [daemonize 0.4.1](https://crates.io/crates/daemonize)
* [systemstat 0.1.8](https://github.com/unrelentingtech/systemstat)
* [sysinfo 0.20.0](https://crates.io/crates/sysinfo)

### How to run
```sh
# Needs to be ran as root
> make build
> sudo make start-daemon
```

### How to find and kill the daemon
```sh
make kill-marvin
```

### Project Progress
- [x] Run application as a daemon
- [x] Monitor disk usage metric as a POC
- [x] Monitor memory usage metric as a POC
- [x] Save data on local file on disk
- [x] Create Makefile
- [x] Write tests
- [ ] Publish on github as open source
- [ ] Import config file from /etc/marvin as TOML format
- [ ] Create install/startup script

### Roadmap
- [ ] Add flags to control daemon
- [ ] Connect to socket server with address at config if flag specified at startup
- [ ] Send metrics to server if threasholds are met.


### Important Links, Sources and Notes
* [The Linux Programming Interface: A Linux and UNIX System Programming Handbook](https://www.amazon.com/Linux-Programming-Interface-System-Handbook/dp/1593272200)
* [Linux Standard Base Core Specification 3.1 - Chapter 21. Users & Groups](https://refspecs.linuxbase.org/LSB_3.1.1/LSB-Core-generic/LSB-Core-generic/usernames.html)
* [See all information about all processes, daemons, etc. running on Mac OS X](https://superuser.com/questions/43157/see-all-information-about-all-processes-daemons-etc-running-on-mac-os-x)
* [Start a rust binary as a systemd daemon](https://stackoverflow.com/questions/63093667/start-a-rust-binary-as-a-systemd-daemon)
  * Examples: https://github.com/unrelentingtech/systemstat/blob/master/examples/info.rs
  * Example: https://github.com/GuillaumeGomez/sysinfo

