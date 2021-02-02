# Minimal repro for issue unmarshaling RFC3339 strings as Time

Add a breakpoint at lines 41|57

After unmarshaling, both `Timestamp`s should have `Some(Time(...))` values

But neither does:

![Screenshot](./timestamps.png)