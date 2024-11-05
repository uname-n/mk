# mk

`mk` is a lightweight and efficient tool designed to manage and run commands at the project level, providing you with a simplified way of handling background tasks and custom command execution.

## Getting Started

### Installation

#### Homebrew
To install `mk` using [homebrew](https://brew.sh), run the following:
```
brew tap uname-n/brew
brew intall mk
```

#### Manual
To install `mk` manually, download the latest [Release](https://github.com/uname-n/mk/releases), make sure to download the correct version for your operating system.

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
command = ["func", "start"]
background_tasks = [
    [
        "azurite",
        "--silent",
        "--location",
        ".azurite",
        "--debug",
        ".azurite/debug.log",
    ],
]

[test]
command = ["echo", "hello"]
```

## Example Output

Running a local command setup will produce output similar to this:

```bash
project % mk test
mk:: running "test"
mk:: [1] background task: ["ping", "localhost"]
mk:: [2] background task: ["ping", "localhost"]
mk:: = = = = = = = = = = = = = = = = = = = = =

PING localhost (127.0.0.1): 56 data bytes
64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.029 ms
64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.054 ms
64 bytes from 127.0.0.1: icmp_seq=2 ttl=64 time=0.273 ms
^C
--- localhost ping statistics ---
3 packets transmitted, 3 packets received, 0.0% packet loss
round-trip min/avg/max/stddev = 0.029/0.119/0.273/0.110 ms

mk:: = = = = = = = = = = = = = = = = = = = = =
mk:: ctrl-c signal recieved. shutting down gracefully...
mk:: = = = = = = = = = = = = = = = = = = = = =
mk:: exiting background task. pid=60455
mk:: exiting background task. pid=60464
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
command = ["func", "start"]
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