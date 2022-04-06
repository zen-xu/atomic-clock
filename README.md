# AtomicClock: Better, Faster dates & times for Python

<div align="left">
    <a href="https://github.com/zen-xu/atomic-clock/actions">
        <img src="https://github.com/zen-xu/atomic-clock/actions/workflows/CI.yml/badge.svg" alt="CI">
    </a>
    <a href="https://results.pre-commit.ci/latest/github/zen-xu/atomic-clock/main">
        <img src="https://results.pre-commit.ci/badge/github/zen-xu/atomic-clock/main.svg">
    </a>
    <a href="https://pypi.org/project/atomic-clock">
        <img alt="PyPI" src="https://img.shields.io/pypi/v/atomic-clock">
    </a>
    <a href="https://pypi.org/project/atomic-clock">
        <img src="https://img.shields.io/pypi/pyversions/atomic-clock">
    </a>
    <a href="https://github.com/zen-xu/atomic-clock/blob/main/LICENSE">
        <img src="https://img.shields.io/pypi/l/atomic-clock.svg">
    </a>
    <a href="https://github.com/psf/black">
        <img src="https://img.shields.io/badge/code%20style-black-000000.svg">
    </a>
</div>

[Arrow](https://github.com/arrow-py/arrow) is a very awesome library for date, time and timezone handling. BUT, it is very slow. Because it is a python builtin `datetime` module wrapper, so it will cost more time to process.

So I create the **AtomicClock**. It implements most of [Arrow](https://github.com/arrow-py/arrow) features, and it is super fast (nearly x10).

One more thing: **AtomicClock** has no other dependencies, it has builtin `Tz` and `RelativeDelta`. So you don't need packages like [pytz](https://pypi.org/project/pytz/) or [dateutil](https://pypi.org/project/python-dateutil/).

## Features
- [x] Fully-implemented, drop-in replacement for datetime
- [x] Support for Python 3.7+
- [x] Timezone-aware and UTC by default
- [x] Super-simple creation options for many common input scenarios
- [x] shift method with support for relative offsets, including weeks
- [x] Format and parse strings automatically
- [x] Wide support for the ISO 8601 standard
- [x] Timezone conversion
- [x] Support for dateutil, pytz, and ZoneInfo tzinfo objects
- [x] Generates time spans, ranges, floors and ceilings for time frames ranging from microsecond to year
- [ ] Humanize dates and times with a growing list of contributed locales (**no plan**)
- [ ] Extensible for your own Arrow-derived types
- [x] Full support for PEP 484-style type hints

## Quick Start

### Installation

```bash
$ pip install -U atomic-clock
```

### Example Usage

```python
>>> import atomic_clock
>>> atomic_clock.get("2022-04-06T23:10:39.503909+08:00")
... <AtomicClock [2022-04-06T23:10:39.503909+08:00]>

>>> utc = atomic_clock.utcnow()
>>> utc
... <AtomicClock [2022-04-06T15:12:36.641201+00:00]>

>>> utc = utc.shift(hours=-1)
>>> utc
... <AtomicClock [2022-04-06T14:12:36.641201+00:00]>

>>> local = utc.to("Asia/Shanghai")
>>> local
... <AtomicClock [2022-04-06T22:12:36.641201+08:00]>

>>> local.timestamp()
... 1649254356.641201

>>> local.format()
... '2022-04-06 22:12:36+08:00'

>>> local.format("%Y-%m-%d %H:%M:%S %:z")
... '2022-04-06 22:12:36 +08:00'
```
