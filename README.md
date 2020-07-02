Productive mind
===============

A productive mind can push and pop tasks into it's stack efficiently
--------------------------------------------------------------------

Push tasks into the mind stack (or continue with an existing task)

```bash
mind

# Enter the names for the tasks to push.
# Press [ENTER] again to save the added tasks.
```

Pop the current task from the mind stack

```bash
mind pop

# Alias
mind p
```

Or while in interactive mode

```bash
/pop

# Alias
/p
```

Supported commands in both CLI and interactive mode

| Command                             | Aliases             | Action
|-------------------------------------|---------------------|------------------------------------------
| `list`                              | `l`                 | List the tasks.
| `pop`                               | `p`                 | Pop out the current task
| `pop {int1}`                        | `p {int}`           | Pop out the task at the given position
| `continue {int}`                    | `c {int}`           | Continue with task at the given position

Example 1: Continue with the task positioned at `[3]`

* CLI mode

```bash
mind c 3
```

* Interactive mode

```bash
/c 3
```

I'll keep adding features (small or big) and keep improving the code quality
while I learn more cool ways to be productive and become better developer.
