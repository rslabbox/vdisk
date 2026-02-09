# vdisk

Virtual Disk Image Tool.

## Usage

Install and run:

```bash
cargo install vdisk
vdisk <PATH> [--part ID|NAME] <COMMAND>
```

The disk image path is positional. Partition selection is optional and uses `--part`.

```bash
vdisk disk.img dd --size 64M
vdisk disk.img gpt -f parameter.txt
vdisk disk.img --part boot mkfs --fstype fat32 
vdisk disk.img --part boot ls /
```

### Commands

- `dd`  Create a blank disk image
- `gpt`  Create GPT partition table using parameter.txt
- `mkfs`   Format filesystem on partition or whole disk
- `ls`     List files in directory
- `cp`     Copy files between host and image
- `mv`     Move/rename files between host and image
- `rm`     Remove file or directory inside image
- `mkdir`  Create directory inside image
- `cat`    Print file content inside image
- `info`   Show disk and partition info

### Common Options

- `--part <ID|NAME>`  Select a partition by index or name
- `-h, --help`        Show help
- `-V, --version`     Show version

## License

Apache-2.0
