## peshming
It's a prometheus exporter pinging hosts in the background.
It's been inspired by [meshping](https://bitbucket.org/Svedrin/meshping),
but instead of managing targets using a redis db this is using a simple config file.
In addition, this tool allows to set a ping frequency per target.

The name peshming is intended as a placeholder until
someone comes up with something better.

### Usage:
```
peshming 0.1.0
Jan Christian Grünhage <jan.christian@gruenhage.xyz>
Pings configured hosts in a configurable intervals and exposes metrics for prometheus.

USAGE:
    peshming [FLAGS] <config>

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Be verbose (you can add this up to 4 times for more logs)
    -V, --version    Prints version information

ARGS:
    <config>    Set config file

```
For configuration options, see the included sample config file.