from enum import IntEnum

from .atomic_clock import AtomicClock
from .atomic_clock import RelativeDelta
from .atomic_clock import Tz
from .atomic_clock import __version__
from .atomic_clock import get
from .atomic_clock import now
from .atomic_clock import utcnow


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
    "RelativeDelta",
    "Tz",
    "Weekday",
    "get",
    "now",
    "utcnow",
    "__version__",
]
