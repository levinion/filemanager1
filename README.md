# FileManager1

This is an implementation of [file-manager-interface](https://www.freedesktop.org/wiki/Specifications/file-manager-interface/).

It could be an alternative to [boydaihungst's implementation](https://github.com/boydaihungst/org.freedesktop.FileManager1.common).

## Installation

```bash
git clone "https://github.com/levinion/filemanager1"
cd filemanager1
make
```

## Configuration

The application searches for the configuration file in the following order:

1. `$XDG_CONFIG_HOME/filemanager1/config.toml`
2. `~/.config/filemanager1/config.toml` 
3. `/etc/filemanager1/config.toml`

If no config file is found, the following default is used:

```toml
cmd = "foot -a yazi -e yazi {}"
```

For example:

As the service receives a call like:

```bash
gdbus call --session \
  --dest org.freedesktop.FileManager1 \
  --object-path /org/freedesktop/FileManager1 \
  --method org.freedesktop.FileManager1.ShowFolders \
  "['file://$HOME/Documents', 'file://$HOME/Downloads']" ""
```

The cmd will be transformed into:

```bash
foot -a yazi -e yazi $HOME/Documents $HOME/Downloads
```

If there's more customized needs, try wrapping the command with a script instead.
