## Usage

```
$ regex2dir -a '12?3+4*\+' ./whatever
$ cd whatever
$ test -e '1/2/3/+/ACCEPT' && echo ACCEPT
ACCEPT
$ test -e '1/3/3/4/4/+/ACCEPT' && echo ACCEPT
ACCEPT
$ test -e '1/4/4/+/ACCEPT' && echo ACCEPT
$ tree
.
└── 1
    ├── 2
    │   └── 3
    │       ├── +
    │       │   └── ACCEPT
    │       ├── 3 -> ~/whatever/1/2/3
    │       └── 4
    │           ├── + -> ~/whatever/1/2/3/+
    │           └── 4 -> ~/whatever/1/2/3/4
    └── 3 -> ~/whatever/1/2/3

9 directories, 1 file
```