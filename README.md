# gid
***GET IT DONE***

A simple command line interface for Google Tasks.

It's not very polished and there are a couple of problems with it so maybe I'll come back to this at some point.

```
~ ❯ gid tasks add "Make Grocery List"
~ ❯ gid tasks add "Go to Store"
~ ❯ gid tasks add "Run a Marathon"
~ ❯ gid tasks list
╭───┬───────────────────╮
│ # │ Name              │
├───┼───────────────────┤
│ 0 │ Run a Marathon    │
│ 1 │ Go to Store       │
│ 2 │ Make Grocery List │
╰───┴───────────────────╯
```

## Install
 -  Create a project from the google developer console... (from what I can remember)
   - Add your email(s) to the test users.
   - Complete the auth page setup.
   - Make sure that the google tasks read/write apis are enabled.
   - Create new client credentials and save them in this build directory as `clientsecret.json`.
  
 - Build and Install
```shell
$ cargo install --path="."
```

On your first run it should provide you a URL to go to so you can log into your google account.

## Configuration
There is a configuration file at `~/.config/gid.toml`:
```toml
default_list = "0" # the index or name that will be used by default by the tasks subcommand
table_style = "rounded" # markdown|empty|blank|ascii|ascii_rounded|modern|sharp|rounded|extended|dots
```

## Help Page
```
Usage: gid [-b <base-url>] [-a <user-agent>] [-c <config-path>] <command> [<args>]

Use Google Tasks from the command line.

Options:
  -b, --base-url    set the base url to use in all requests to the server
  -a, --user-agent  set the user-agent header field to use in all requests to
                    the server
  -c, --config-path custom path to a config file
  --help            display usage information

Commands:
  lists             manage task lists
  tasks             manage tasks
```