from enum import IntEnum

from _atomic_clock import AtomicClock
from _atomic_clock import Tz
from _atomic_clock import __version__
from _atomic_clock import get
from _atomic_clock import now
from _atomic_clock import utcnow


class Weekday(IntEnum):
    Mon = 0
    Tue = 1
    Wed = 2
    Thu = 3
    Fri = 4
    Sat = 5
    Sun = 6


__all__ = [
    "AtomicClock",
    "Tz",
    "Weekday",
    "get",
    "now",
    "utcnow",
    "__version__",
]
