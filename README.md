# 🗝️ `chest`

> [!CAUTION]
> This program is provided as is and has not been audited.
> 
> Use it to encrypt important files at your own risk.

Chest is a simple CLI program to encrypt and store files in a single binary blob (chest). For example, having a single file is handy when storing on public cloud platforms.

## Usage

```bash
A file encryption CLI tool

Usage: chest <COMMAND>

Commands:
  new   Create a new chest
  peek  Peek into a chest and list its content, decrypting only metadata
  open  Open a chest and extract its encrypted content
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Create a chest

Create a new chest and add files with `chest new`.

This command will create a new `.chest` file containing your compressed and encrypted files and the metadata needed to extract them later.

Example:

```bash
chest new top-secret --add nuclear-launch-codes.txt --add presidential-bunker-geoloc.txt
```

### Inspect a chest

Inspect an existing `.chest` file with `chest peek` and display its metadata (algorithms used, file list, etc.)

Example:

```bash
chest peek top-secret.chest
```

### Decrypt and extract chest files

Decrypt and extract the files from a chest with `chest open`.

The command extracts the files in a directory named after the `.chest` file or an arbitrary name via the `--out` flag.

Example:

```bash
chest open top-secret.chest
```

