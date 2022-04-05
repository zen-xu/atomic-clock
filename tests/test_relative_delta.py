import pytest

from atomic_clock import AtomicClock
from atomic_clock import RelativeDelta


@pytest.mark.parametrize(
    "dt,delta,expected",
    (
        (AtomicClock(2022, 4, 1), RelativeDelta(years=1), AtomicClock(2023, 4, 1)),
        (AtomicClock(2022, 4, 1), RelativeDelta(months=1), AtomicClock(2022, 5, 1)),
        (AtomicClock(2022, 4, 1), RelativeDelta(days=1), AtomicClock(2022, 4, 2)),
        (AtomicClock(2022, 4, 1), RelativeDelta(hours=1), AtomicClock(2022, 4, 1, 1)),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(minutes=1),
            AtomicClock(2022, 4, 1, 0, 1),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(seconds=1),
            AtomicClock(2022, 4, 1, 0, 0, 1),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(microseconds=1),
            AtomicClock(2022, 4, 1, 0, 0, 0, 1),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(quarters=1),
            AtomicClock(2022, 7, 1),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weeks=1),
            AtomicClock(2022, 4, 8),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weekday=0),
            AtomicClock(2022, 4, 4),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weekday=1),
            AtomicClock(2022, 4, 5),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weekday=2),
            AtomicClock(2022, 4, 6),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weekday=3),
            AtomicClock(2022, 4, 7),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weekday=4),
            AtomicClock(2022, 4, 1),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weekday=5),
            AtomicClock(2022, 4, 2),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(weekday=6),
            AtomicClock(2022, 4, 3),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(days=-30),
            AtomicClock(2022, 3, 2),
        ),
        (
            AtomicClock(2022, 4, 1),
            RelativeDelta(years=1, days=-30),
            AtomicClock(2023, 3, 2),
        ),
    ),
)
def test_relative_delta(dt, delta, expected):
    assert dt + delta == expected
