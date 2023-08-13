# raisinit
LibC-free init binary, runs commands from `/etc/raisinrc` separated by newlines.

The maximum size of raisinrc is 2048 bytes.

Programs run by raisinit do not automatically respawn when terminated.

## respawn
Runs a program passed to it, respawning it if it terminates.