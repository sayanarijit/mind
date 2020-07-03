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
| `{int}`                             |                     | Continue with the task at the given position
| `pop`                               | `p`                 | Pop out the current task
| `pop {int}`                         | `p {int}`           | Pop out the task at the given position

Example 1: Continue with the task positioned at `[3]`

* CLI mode

```bash
mind 3
```

* Interactive mode

```bash
/3
```

Example 2: Pop the task positioned at `[3]`

* CLI mode

```bash
mind p 3
```

* Interactive mode

```bash
/p 3
```

A productive mind can remind itself of the pending and repeating tasks
----------------------------------------------------------------------

Open `~/.mind/mind.yml` and add the reminders in the given format

```yaml
reminders:
  - name: Test reminder once
    when: "2020-07-10T23:09:42.233022+05:30"
    repeat: Never

  - name: Test reminder everyday
    when: "2020-07-04T23:10:42.233022+05:30"
    repeat: EveryDay

  - name: Test reminder every week
    when: "2020-07-10T23:09:42.233022+05:30"
    repeat: EveryWeek
  
  - name: Test reminder every saturday and sunday
    when: "2020-07-11T00:08:00.233022+05:30"
    repeat:
      Weekly:
        - Sat
        - Sun

  - name: Test reminder every year
    when: "2020-07-10T23:09:42.233022+05:30"
    repeat: EveryYear
```

I'll keep adding features (small or big) and keep improving the code quality
while I learn more cool ways to be productive and become better developer.
