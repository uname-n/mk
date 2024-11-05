# mk

`mk` is a lightweight and efficient tool designed to manage and run commands at the project level, providing you with a simplified way of handling background tasks and custom command execution.

## Getting Started

### Installation
To install `mk`, download the latest [Release](https://github.com/uname-n/mk/releases), make sure to download the correct version for your operating system.

- Make the file executable
    - `chmod +x mk`
- Move the file to a directory in your PATH
    - `mv mk /usr/local/bin`

### Usage

To instruct `mk` to perform a task, use the syntax:

```bash
mk <command>
```

### Configuration

`mk` is configured using a TOML file, which specifies the commands and the background tasks associated with them. Here's an example configuration:

```toml
[local]
command = [["func", "start"]]
background_tasks = [
    [
        "azurite",
        "--silent",
        "--location",
        ".azurite",
        "--debug",
        "$AZURITE_DEBUG_PATH",
    ],
]

[test]
command = [["ping", "localhost"]]
background_tasks = [["ping", "localhost"], ["ping", "localhost"]]
```

## Example Output

Running a local command setup will produce output similar to this:

```bash
project % mk test
mk:: running "test"
mk:: [1] background task: ["ping", "localhost"]
mk:: [2] background task: ["ping", "localhost"]

mk:: running ["ping", "localhost"]
mk:: = = = = = = = = = = = = = = = = = = = = =
PING localhost(localhost (::1)) 56 data bytes
64 bytes from localhost (::1): icmp_seq=1 ttl=64 time=0.013 ms
64 bytes from localhost (::1): icmp_seq=2 ttl=64 time=0.042 ms
64 bytes from localhost (::1): icmp_seq=3 ttl=64 time=0.013 ms
^C
--- localhost ping statistics ---
3 packets transmitted, 3 received, 0% packet loss, time 2047ms
rtt min/avg/max/mdev = 0.013/0.022/0.042/0.013 ms

mk:: = = = = = = = = = = = = = = = = = = = = =
mk:: ctrl-c signal received. Shutting down...
mk:: = = = = = = = = = = = = = = = = = = = = =
mk:: exiting background task. pid=28009
mk:: exiting background task. pid=28010
mk:: = = = = = = = = = = = = = = = = = = = = =
mk:: done 
```

## Environment Variables Support

`mk` can utilize environment variables within the configuration. You can set the variables in your environment and then reference them in `mk.toml` as shown below:

Set environment variables in `.env`:

```bash
AZURITE_DEBUG_PATH=.azurite/debug.log
```

Reference in `mk.toml` configuration:

```toml
[local]
command = [["func", "start"]]
background_tasks = [
    [
        "azurite",
        "--silent",
        "--location",
        ".azurite",
        "--debug",
        "$AZURITE_DEBUG_PATH",
    ],
]
```

## Troubleshooting & Support

For troubleshooting assistance or additional support, please refer to the documentation or raise an issue on the project's GitHub repository.
