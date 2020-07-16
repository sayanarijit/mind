A productive mind has an empty stack
===================================

[![Crates.io](https://img.shields.io/crates/v/mind.svg)](https://crates.io/crates/mind)

[![asciicast](https://asciinema.org/a/345440.svg)](https://asciinema.org/a/345440)

The philosophy
--------------

[mind](https://github.com/sayanarijit/mind) follows the following philosophy

> ***A productive mind has an empty stack.***

Explaination:

Sometimes we have too much on our mind but neither the traditional check boxes, nor
the kanban board works for us. This is because our mind executes the tasks in
[LIFO](https://en.wikipedia.org/wiki/LIFO_(education)) approach like a
[stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)).

The longer we hold a task in the stack of our mind, the more productivity it will
lose. Also, trying multitasking with this stack can cause unpredictable results.

We need to execute them as early as possible. But one by one.

[mind](https://github.com/sayanarijit/mind) makes it easy to work with the stack
of your own mind. It uses this simple formula to measure the current productivity
level of our mind and uses the appropriate colors to represent the it.

> ***p = O - b***
>
> Where ***p*** is productivity, ***O*** is the optimal (desired) productivity,
> and ***b*** is backlog.

In other words, the more tasks you keep on your mind and the longer you keep them
there, the less productive you will become.

You want to keep this stack empty.

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

| Command         | Aliases     | Action
|-----------------|-------------|------------------------------------------
| `{num}`         |             | Continue with the task at the given position
| `pop`           | `p`         | Pop out the current task
| `pop {num}`     | `p {num}`   | Pop out the task at the given position
| `edit`          | `e`         | Edit the current task
| `edit {num}`    | `e {num}`   | Edit the task at the given position
| `get`           | `g`         | Get details of the current task
| `get {num}`     | `g {num}`   | Get details of the task at the given position

Examples
--------

Example 1: Add all the `TODO` and `FIXME` items from the codebase.

```bash
grep -nR TODO . | mind
grep -nR FIXME . | mind
```

Example 2: Continue with the task positioned at `[3]`

* CLI mode

```bash
mind 3
```

* Interactive mode

```bash
/3
```

Example 3: Pop the task positioned at `[3]`

* CLI mode

```bash
mind p 3
```

* Interactive mode

```bash
/p 3
```

Example 4: Edit the task positioned at `[3]`

* CLI mode

```bash
mind e 3
```

* Interactive mode

```bash
/e 3
```

Example 5: Get details of the task positioned at `[3]`

* CLI mode

```bash
mind g 3
```

* Interactive mode

```bash
/g 3
```

A productive mind can remind itself of the pending and repeating tasks
----------------------------------------------------------------------

Open `~/.mind/reminders.yml` and add the reminders in the given format (see `~/.mind/reminder_examples.yml`)

```yaml
# This reminder will disappear once executed.

- name: Test reminder once on 10 July 2020, at 8 am IST
  when: "2020-07-10T08:00:00+05:30"
  repeat: Never

# The following reminders will reschedule themselves.

- name: "Test reminder everyday at 10:30 pm IST"
  when: "2020-07-10T22:30:00+05:30"
  repeat: EveryDay

- name: "Test reminder every other day at 10:30 pm IST"
  when: "2020-07-10T22:30:00+05:30"
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
while I learn more cool ways to be productive and become a better developer.
