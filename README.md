# Marvin - Metrics Tracker

### Requirements
* [daemonize — A tool to run a command as a daemon](http://software.clapper.org/daemonize/#installation)
* [daemonize 0.4.1](https://crates.io/crates/daemonize)
* [systemstat 0.1.8](https://github.com/unrelentingtech/systemstat)

### How to run
```
# Needs to be ran as root
./target/debug/marvin
```

### How to find and kill the daemon
```
ps -A | grep marvin
kill -9 <pid>
```

### Project Progress
- [x] Run application as a daemon
- [x] Monitor disk usage metric as a POC
- [x] Monitor memory usage metric as a POC
- [x] Save data on local file on disk
- [ ] Write tests
- [ ] Import config file from /etc/marvin as TOML format
- [ ] Create install/startup script
- [ ] Publish on github as open source

### Roadmap
- [ ] Add flags to control daemon
- [ ] Connect to socket server with address at config
- [ ] Create Makefile

### Important Links, Sources and Notes
* [The Linux Programming Interface: A Linux and UNIX System Programming Handbook](https://www.amazon.com/Linux-Programming-Interface-System-Handbook/dp/1593272200)
* [Linux Standard Base Core Specification 3.1 - Chapter 21. Users & Groups](https://refspecs.linuxbase.org/LSB_3.1.1/LSB-Core-generic/LSB-Core-generic/usernames.html)
* [See all information about all processes, daemons, etc. running on Mac OS X](https://superuser.com/questions/43157/see-all-information-about-all-processes-daemons-etc-running-on-mac-os-x)
* [Start a rust binary as a systemd daemon](https://stackoverflow.com/questions/63093667/start-a-rust-binary-as-a-systemd-daemon)
  * Examples: https://github.com/unrelentingtech/systemstat/blob/master/examples/info.rs

