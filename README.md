A Productive Mind
=================

[![Crates.io](https://img.shields.io/crates/v/mind.svg)](https://crates.io/crates/mind)

[![asciicast](https://asciinema.org/a/345280.svg)](https://asciinema.org/a/345280)

Install
-------

You need [cargo to install mind](https://www.rust-lang.org/tools/install).

```bash
cargo install mind
```

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

Open `~/.mind/mind.yml` and add the reminders in the given format (see `~/.mind/reminder_examples.yml`)

```yaml
reminders:

  # This reminder will disappear once executed.

  - name: Test reminder once on 10 July 2020, at 8 am IST
    when: "2020-07-10T08:00:00+05:30"
    repeat: Never

  # Following reminders will reschedule themselves.

  - name: "Test reminder everyday at 10:30 pm IST"
    when: "2020-07-10T10:30:00+05:30"
    repeat: EveryDay

  - name: "Test reminder every other day at 10:30 pm IST"
    when: "2020-07-10T10:30:00+05:30"
    repeat:
      EveryNthDay: 2

  - name: Test reminder every week at 11 am IST
    when: "2020-07-10T11:00:00+05:30"
    repeat: EveryWeek

  - name: Test reminder every 3rd week at 11 am IST
    when: "2020-07-10T11:00:00+05:30"
    repeat:
      EveryNthWeek: 3

  - name: "Test reminder every saturday and sunday at 9:15 am IST"
    when: "2020-07-10T09:15:00+05:30"
    repeat:
      Weekdays:
        - Sat
        - Sun

  - name: "Test reminder every 2nd saturday at 9:15 am IST"
    when: "2020-07-10T09:15:00+05:30"
    repeat:
      EveryNthWeekday:
        n: 2
        weekday: Sat
```

I'll keep adding features (small or big) and keep improving the code quality
while I learn more cool ways to be productive and become better developer.
