## What is kvmm?

kvmm is a tool that can save in K-V type on the command line

## Installation

**for x86 linux**

Install kvmm

```
wget https://github.com/ushinohama966/kvmm/releases/download/[version]/kvmm.tar.gz
tar -zxvf kvmm.tar.gz
rm kvmm.tar.gz
```

Set environment variable

```
export MEMO_FILE_PATH=[path/to/memo.json]
```

## Samples

Add a value

```
> kvmm add -k name -v john
add >>> {"name":"john"}
```

List values

```
> kvmm list
{"name":"john"}
```
