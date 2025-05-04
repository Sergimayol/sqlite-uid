# sqlite-uid

SQLite3 extension for Unique Unique Identifier (UID).

## Run Demo

1. Compile:

```bash
cargo build --release
```

2. Run:

```bash
sqlite3 :memory: '.read tests/test.sql'
```

3. Output:

```bash
┌────────────────────────────┐
│            ulid            │
├────────────────────────────┤
│ 01JTE01QSS10JPAT14697H2JZY │
└────────────────────────────┘
Run Time: real 0.000 user 0.000017 sys 0.000017
┌────────────────────────────┐
│            ulid            │
├────────────────────────────┤
│ 01JTE01QSSP2HDM56B5WA822DD │
└────────────────────────────┘
Run Time: real 0.000 user 0.000004 sys 0.000004
┌────────────────────────────┐
│             id             │
├────────────────────────────┤
│ 01JTE01QSS289G3RQ3QT7Y16DR │
│ 01JTE01QSS33GTHKVNQ86VARC4 │
│ 01JTE01QSS36PSQZW5FAVGAGXE │
│ 01JTE01QSSK1MDG4JCAV25Z3QX │
└────────────────────────────┘
Run Time: real 0.000 user 0.000015 sys 0.000015
┌──────────────────────────┐
│        object_id         │
├──────────────────────────┤
│ 68178d89a71fcbf96b2ed7b4 │
└──────────────────────────┘
Run Time: real 0.000 user 0.000005 sys 0.000005
┌──────────────────────────┐
│            id            │
├──────────────────────────┤
│ 68178d89a71fcbf96b2ed7b5 │
│ 68178d89a71fcbf96b2ed7b6 │
│ 68178d89a71fcbf96b2ed7b7 │
│ 68178d89a71fcbf96b2ed7b8 │
└──────────────────────────┘
Run Time: real 0.000 user 0.000009 sys 0.000009
```
