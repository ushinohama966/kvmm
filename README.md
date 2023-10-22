## What is kvmm?

kvmm is a tool that can save in K-V type on the command line

## Installation

**for linux**

Install kvmm

```
wget https://github.com/ushinohama966/kvmm/releases/download/[version]/kvmm-[target].tar.gz
tar -zxvf kvmm-[target].tar.gz
rm kvmm-[target].tar.gz
```

Set environment variable

```
export MEMO_FILE_PATH=[path/to/memo.json]
```

## Samples

Add a value

```
> kvmm add -k name -v john
1 items found.
---
key
```

List values

```
> kvmm list
{"name":"john"}
```
