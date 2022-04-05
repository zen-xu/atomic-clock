import time

from datetime import datetime

import atomic_clock
import pytest

from dateutil import tz


@pytest.fixture(scope="class")
def time_utcnow(request):
    timestamp = time.time()
    request.cls.atomic_clock = atomic_clock.AtomicClock.fromtimestamp(timestamp, tz.UTC)
    request.cls.now = datetime.fromtimestamp(timestamp, tz.UTC)


@pytest.fixture(scope="class")
def time_2013_01_01(request):
    request.cls.now = atomic_clock.utcnow()
    request.cls.atomic_clock = atomic_clock.AtomicClock(2013, 1, 1)
    request.cls.datetime = datetime(2013, 1, 1)


@pytest.fixture(scope="class")
def time_2013_02_03(request):
    request.cls.atomic_clock = atomic_clock.AtomicClock(2013, 2, 3, 12, 30, 45, 1)


@pytest.fixture(scope="class")
def time_2013_02_15(request):
    request.cls.datetime = datetime(2013, 2, 15, 3, 41, 22, 8923)
    request.cls.atomic_clock = atomic_clock.AtomicClock.fromdatetime(
        request.cls.datetime
    )


@pytest.fixture(scope="class")
def time_1975_12_25(request):
    request.cls.datetime = datetime(
        1975, 12, 25, 14, 15, 16, tzinfo=tz.gettz("America/New_York")
    )
    request.cls.atomic_clock = atomic_clock.fromdatetime(request.cls.datetime)
