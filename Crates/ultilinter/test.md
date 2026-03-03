---
author: John Doe
date: 2026-02-26
---

# Bad Header
## AnotherBadHeader

This is a paragraph with trailing spaces.    
    

This paragraph is separated by too many blank lines.


###Good Header

Some text.

    

---

## What This File Violates

This file triggers:

### MD001 – Trailing Whitespace
- Lines with extra spaces at the end
- Blank lines that contain spaces

### MD002 – Multiple Consecutive Blank Lines
- More than one empty line in a row

### MD003 – Missing Space After `#`
- `#Bad Header`
- `##AnotherBadHeader`

### MD004 – Missing `title:` in Front Matter
- Front matter exists but does not contain `title:`

---

If you run:

```bash
cargo run --example cli example.md
```