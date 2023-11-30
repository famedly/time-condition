# `time-condition`

`time-condition` is a command line utility for checking whether the current
time matches a condition. It might be useful to narrow down cases in which a
certain action should be performed when running inside a cron job for example.

## Installation

Use the package manager
[cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to
install `time-condition`.

```bash
cargo install time-condition
```

## Usage

```bash
# Returns a positive result when run on a Thursday in even iso weeks.
time-condition 'iso_week % 2 == 0 && week_day == 4'
# Same but for Sundays in odd iso weeks.
time-condition 'iso_week % 2 == 1 && week_day == 7'
```

The expression evaluation is powered by
https://docs.rs/evalexpr/latest/evalexpr/, with a few additional time based
variables exposed:
- **year**: The current year.
- **month**: The current month (`1..=12`).
- **iso_week**: The current week in the ISO week-numbering year (`1..=53`).
- **day**: The current day of the month (`1..=31`).
- **week_day**: The day in the week, one-indexed, starting with Monday (`1..=7`).
- **hour**: The current hour (`0..=23`).
- **minute**: The current minute (`0..=60`).

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[AGPL-3.0](https://choosealicense.com/licenses/agpl-3.0/)
