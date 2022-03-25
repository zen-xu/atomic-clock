from __future__ import annotations

import datetime as dt

from enum import IntEnum
from time import struct_time
from typing import Literal
from typing import Optional
from typing import Tuple

class Weekday(IntEnum):
    Mon = 0
    Tue = 1
    Wed = 2
    Thu = 3
    Fri = 4
    Sat = 5
    Sun = 6

class AtomicClock:
    """An :class:`AtomicClock <atomic_clock.AtomicClock>` object.

    Implements the ``datetime`` interface, behaving as an aware ``datetime`` while implementing
    additional functionality.

    :param year: the calendar year.
    :param month: the calendar month.
    :param day: the calendar day.
    :param hour: (optional) the hour. Defaults to 0.
    :param minute: (optional) the minute, Defaults to 0.
    :param second: (optional) the second, Defaults to 0.
    :param microsecond: (optional) the microsecond. Defaults to 0.
    :param tzinfo: (optional) A timezone expression.  Defaults to UTC.

    .. _tz-expr:

    Recognized timezone expressions:
        - A ``tzinfo`` object (note: very slow).
        - A ``atomic_clock.Tz`` object.
        - A ``str`` describing a timezone, similar to 'US/Pacific', or 'Asia/Shanghai'.
        - A ``str`` in ISO 8601 style, as in '+07:00'.
        - A ``str``, one of the following:  'local', 'utc', 'UTC'.

    Usage::
        >>> import atomic_clock
        >>> atomic_clock.AtomicClock(2022, 3, 20, 10, 30, 45)
        <AtomicClock [2022-03-20T10:30:45+00:00]>

    """

    def __init__(
        self,
        year: int,
        month: int,
        day: int,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        microsecond: int = 0,
        tzinfo: str | dt.tzinfo = "local",
    ) -> None: ...
    @classmethod
    def now(cls, tzinfo: str | dt.tzinfo | Tz = "local") -> AtomicClock:
        """Constructs an :class:`AtomicClock <atomic_clock.AtomicClock>` object, representing "now" in the given
        timezone.

        :param tzinfo: (optional) A timezone expression. Defaults to local time.

        .. _tz-expr:

        Recognized timezone expressions:
            - A ``tzinfo`` object (note: very slow).
            - A ``atomic_clock.Tz`` object.
            - A ``str`` describing a timezone, similar to 'US/Pacific', or 'Asia/Shanghai'.
            - A ``str`` in ISO 8601 style, as in '+07:00'.
            - A ``str``, one of the following:  'local', 'utc', 'UTC'.

        Usage::
            >>> AtomicClock.now('Asia/Shanghai')
            <AtomicClock [2022-03-21T12:57:43.324231+08:00]>
        """
    @classmethod
    def utcnow(cls) -> AtomicClock:
        """Constructs an :class:`AtomicClock <atomic_clock.AtomicClock>` object, representing "now" in UTC
        timezone.

        Usage::
            >>> AtomicClock.utcnow()
            <AtomicClock [2022-03-21T04:58:42.796864+00:00]>
        """
    @classmethod
    def fromtimestamp(
        cls,
        timestamp: float,
        tzinfo: str | dt.tzinfo | Tz = "local",
    ) -> AtomicClock:
        """Constructs an :class:`AtomicClock <atomic_clock.AtomicClock>` object from a timestamp, converted to
        the given timezone.

        :param timestamp: an float that converts to.
        :param tzinfo: (optional) A timezone expression. Defaults to local time.

        .. _tz-expr:

        Recognized timezone expressions:
            - A ``tzinfo`` object (note: very slow).
            - A ``atomic_clock.Tz`` object.
            - A ``str`` describing a timezone, similar to 'US/Pacific', or 'Asia/Shanghai'.
            - A ``str`` in ISO 8601 style, as in '+07:00'.
            - A ``str``, one of the following:  'local', 'utc', 'UTC'.
        """
    @classmethod
    def utcfromtimestamp(
        cls,
        timestamp: float,
    ) -> AtomicClock:
        """Constructs an :class:`AtomicClock <atomic_clock.AtomicClock>` object from a timestamp in UTC time

        :param timestamp: an float that converts to.
        """
    @classmethod
    def fromdatetime(
        cls, dt: dt.datetime, tzinfo: str | dt.tzinfo | Tz = "utc"
    ) -> AtomicClock:
        """Constructs an :class:`atomic_clock <atomic_clock.AtomicClock>` object from a ``datetime`` and
        optional replacement timezone.

        :param dt: the ``datetime``
        :param tzinfo: (optional) A :ref:`timezone expression <tz-expr>`.  Defaults to ``dt``'s
            timezone, or UTC if naive.

        .. _tz-expr:

        Recognized timezone expressions:
            - A ``tzinfo`` object (note: very slow).
            - A ``atomic_clock.Tz`` object.
            - A ``str`` describing a timezone, similar to 'US/Pacific', or 'Asia/Shanghai'.
            - A ``str`` in ISO 8601 style, as in '+07:00'.
            - A ``str``, one of the following:  'local', 'utc', 'UTC'.

        Usage::
            >>> dt
            datetime.datetime(2022, 3, 22, 0, 39, 2, 316809, tzinfo=tzfile('/usr/share/zoneinfo/Asia/Shanghai'))
            >>> AtomicClock.fromdatetime(dt)
            <AtomicClock [2022-03-22T00:39:02.316809+08:00]>
        """
    @classmethod
    def fromdate(cls, dt: dt.date, tzinfo: str | dt.tzinfo | Tz = "utc") -> AtomicClock:
        """Constructs an :class:`AtomicClock <atomic_clock.AtomicClock>` object from a ``date`` and optional
        replacement timezone.  All time values are set to 0.

        :param date: the ``date``
        :param tzinfo: (optional) A :ref:`timezone expression <tz-expr>`.  Defaults to UTC.

        .. _tz-expr:

        Recognized timezone expressions:
            - A ``tzinfo`` object (note: very slow).
            - A ``atomic_clock.Tz`` object.
            - A ``str`` describing a timezone, similar to 'US/Pacific', or 'Asia/Shanghai'.
            - A ``str`` in ISO 8601 style, as in '+07:00'.
            - A ``str``, one of the following:  'local', 'utc', 'UTC'.
        """
    @classmethod
    def strptime(
        cls, date_str: str, fmt: str, tzinfo: str | dt.tzinfo | Tz | None = None
    ) -> AtomicClock:
        """Constructs an :class:`AtomicClock <atomic_clock.AtomiClock>` object from a date string and format,
        in the style of ``datetime.strptime``.  Optionally replaces the parsed timezone.

        :param date_str: the date string.
        :param fmt: the format string using datetime format codes.
        :param tzinfo: (optional) A :ref:`timezone expression <tz-expr>`.  Defaults to the parsed
            timezone if ``fmt`` contains a timezone directive, otherwise UTC.

        Usage::
            >>> AtomicClock.strptime('20-01-2019 15:49:10', '%d-%m-%Y %H:%M:%S')
            <AtomicClock [2019-01-20T15:49:10+00:00]>
        """
    @classmethod
    def fromordinal(cls, ordinal) -> AtomicClock:
        """Constructs an :class:`AtomicClock <atomic_clock.AtomiClock>` object corresponding
        to the Gregorian Ordinal.

        :param ordinal: an ``int`` corresponding to a Gregorian Ordinal.

        Usage::
            >>> AtomicClock.fromordinal(738236)
            <AtomicClock [2022-03-22T00:00:00+00:00]>
        """
    def date(self) -> dt.date:
        """Returns a ``date`` object with the same year, month and day.

        Usage::
            >>> AtomicClock.utcnow().date()
            datetime.date(2022, 3, 22)
        """
    def time(self) -> dt.time:
        """Returns a ``time`` object with the same hour, minute, second, microsecond.

        Usage::
            >>> AtomicClock.utcnow().time()
            datetime.time(5, 52, 33, 354046)
        """
    def timetz(self) -> dt.time:
        """Returns a ``time`` object with the same hour, minute, second, microsecond and
        tzinfo.
        Usage::
            >>> AtomicClock.utcnow().timetz()
            datetime.time(13, 54, 18, 886227, tzinfo=<Tz [UTC]>)
        """
    def astimezone(self, tz: str | dt.tzinfo | Tz | None = None) -> dt.datetime:
        """Returns a ``datetime`` object, converted to the specified timezone.

        :param tz: A :ref:`timezone expression <tz-expr>`.

        Usage::
            >>> shanghai = AtomicClock.now('Asia/Shanghai')
            >>> nyc = AtomicClock.now('America/New_York').tzinfo
            >>> shanghai.astimezone(nyc)
            datetime.datetime(2022, 3, 23, 10, 13, 13, 211622, tzinfo=<Tz [America/New_York]>)
        """
    def utcoffset(self) -> Optional[dt.timedelta]:
        """Returns a ``timedelta`` object representing the whole number of minutes difference from
        UTC time.

        Usage::
            >>> AtomicClock.now('US/Pacific').utcoffset()
            datetime.timedelta(days=-1, seconds=61200)
        """
    def dst(self) -> Optional[dt.timedelta]:
        """Returns the daylight savings time adjustment.

        Usage::
            >>> AtomicClock.now('US/Pacific').dst()
            datetime.timedelta(seconds=3600)
        """
    def timetuple(self) -> struct_time:  # type: ignore
        """Returns a ``time.struct_time``, in the current timezone.

        Usage::
            >>> AtomicClock.utcnow().timetuple()
            time.struct_time(tm_year=2022, tm_mon=3, tm_mday=23, tm_hour=16, tm_min=37, tm_sec=47, tm_wday=2, tm_yday=82, tm_isdst=0)
        """
    def utctimetuple(self) -> struct_time:
        """Returns a ``time.struct_time``, in UTC time.

        Usage::
            >>> AtomicClock.utcnow().utctimetuple()
            time.struct_time(tm_year=2022, tm_mon=3, tm_mday=23, tm_hour=16, tm_min=38, tm_sec=23, tm_wday=2, tm_yday=82, tm_isdst=0)
        """
    def toordinal(self) -> int:
        """Returns the proleptic Gregorian ordinal of the date.

        Usage::
            >>> AtomicClock.utcnow().toordinal()
            738237
        """
    def weekday(self) -> int:
        """Returns the day of the week as an integer (0-6).

        Usage::
            >>> AtomicClock.utcnow().weekday()
            3
        """
    def isoweekday(self) -> int:
        """Returns the ISO day of the week as an integer (1-7).

        Usage::
            >>> AtomicClock.utcnow().isoweekday()
            6
        """
    def isocalendar(self) -> Tuple[int, int, int]:
        """Returns an IsoCalendarDate namedtuple, (ISO year, ISO week number, ISO weekday).
        Usage::
            >>> AtomicClock.utcnow().isocalendar()
            IsoCalendarDate(year=2022, week=12, weekday=3)
        """
    def isoformat(
        self,
        sep: str = "T",
        timespec: Literal[
            "auto", "hours", "minutes", "seconds", "milliseconds", "microseconds"
        ] = "auto",
    ) -> str:
        """Returns an ISO 8601 formatted representation of the date and time.

        Usage::
            >>> AtomicClock.utcnow().isoformat()
            '2022-03-23T16:43:23.314834+00:00'
        """
    def ctime(self) -> str:
        """Returns a ctime formatted representation of the date and time.

        Usage::
            >>> AtomicClock.utcnow().ctime()
            'Wed Mar 23 16:44:00 2022'
        """
    def strftime(self, format: str) -> str:
        """Formats in the style of ``datetime.strftime``.

        :param format: the format string.

        Usage::
            >>> AtomicClock.utcnow().strftime('%d-%m-%Y %H:%M:%S')
            '23-03-2022 16:44:37'
        """
    def clone(self) -> AtomicClock:
        """Returns a new :class:`AtomicClock <atomic_clock.AtomiClock>` object, cloned from the current one.

        Usage:
            >>> now = AtomicClock.utcnow()
            >>> cloned = now.clone()
        """
    def replace(
        self,
        *,
        year: int | None = None,
        month: int | None = None,
        day: int | None = None,
        hour: int | None = None,
        minute: int | None = None,
        second: int | None = None,
        microsecond: int | None = None,
        tzinfo: str | dt.tzinfo | Tz | None = None,
    ) -> None:
        """Returns a new :class:`AtomicClock <atomic_clock.AtomicClock>` object with attributes updated
        according to inputs.

        Use property names to set their value absolutely::

            >>> from atomic_clock import AtomicClock
            >>> now = AtomicClock.utcnow()
            >>> now
            <AtomicClock [2022-03-24T14:44:51.560065+00:00]>
            >>> arw.replace(year=2021, month=8)
            <AtomicClock [2021-08-24T14:44:51.560065+00:00]>

        You can also replace the timezone without conversion, using a
        :ref:`timezone expression <tz-expr>`::

            >>> now.replace(tzinfo="local")
            <AtomicClock [2021-08-24T22:44:51.560065+08:00]>
        """
    def shift(
        self,
        *,
        years: int = 0,
        months: int = 0,
        days: int = 0,
        hours: int = 0,
        minutes: int = 0,
        seconds: int = 0,
        microseconds: int = 0,
        weeks: int = 0,
        quarters: int = 0,
        weekday: Literal[0, 1, 2, 3, 4, 5, 6] | Weekday | None = None,
    ) -> AtomicClock:
        """Returns a new :class:`AtomicClock <atomic_clock.AtomicClock>` object with attributes updated
        according to inputs.

        Use pluralized property names to relatively shift their current value:

        >>> from atomic_clock import AtomicClock
        >>> now = AtomicClock.utcnow()
        >>> now
        <AtomicClock [2022-03-25T10:29:11.634832+00:00]>
        >>> arw.shift(years=1, months=-1)
        <AtomicClock [2023-02-25T10:29:11.634832+00:00]>

        Day-of-the-week relative shifting can use either Python's weekday numbers
        (Monday = 0, Tuesday = 1 .. Sunday = 6) or using day instances (Mon, Tue .. Sun).
        When using weekday numbers, the returned date will always be greater than or equal
        to the starting date.

        Using the above code (which is a Saturday) and asking it to shift to Saturday:

        >>> now.shift(weekday=5)
        <AtomicClock [2022-03-26T10:29:11.634832+00:00]>

        While asking for a Monday:

        >>> now.shift(weekday=0)
        <AtomicClock [2022-03-28T10:29:11.634832+00:00]>
        """
    def for_json(self) -> str:
        """Serializes for the ``for_json`` protocol of simplejson.

        Usage::
            >>> AtomicClock.utcnow().for_json()
            '2022-03-23T16:45:17.722416+00:00'
        """
    def to(self, tzinfo: str | dt.tzinfo | Tz) -> AtomicClock:
        """Returns a new :class:`AtomicClock <atomic_clock.AtomiClock>` object, converted
        to the target timezone.

        :param tzinfo: A :ref:`timezone expression <tz-expr>`.

        Usage::
            >>> utc = AtomicClock.utcnow()
            >>> utc
            <AtomicClock [2022-03-23T12:36:32.198831+00:00]>
            >>> utc.to('US/Pacific')
            <AtomicClock [2022-03-23T05:36:32.198831-07:00]>
            >>> utc.to(tz.tzlocal())
            <AtomicClock [2022-03-23T20:36:32.198831+08:00]>
            >>> utc.to(Tz("local"))
            <AtomicClock [2022-03-23T20:36:32.198831+08:00]>
            >>> utc.to('local')
            <AtomicClock [2022-03-23T20:36:32.198831+08:00]>
            >>> utc.to('-07:00')
            <AtomicClock [2022-03-23T05:36:32.198831-07:00]>
            >>> utc.to('local').to('utc')
            <Arrow [2013-05-09T03:49:12.311072+00:00]>
        """
    def format(self, fmt: str = "%Y-%m-%d %H:%M:%S%Z") -> str:
        """Returns a string representation of the :class:`Arrow <arrow.arrow.Arrow>` object,
        formatted according to the provided format string.

        Visit https://docs.rs/chrono/latest/chrono/format/strftime/index.html to get more formatter details.

        :param fmt: the format string.

        Usage::
            >>> now = AtomicClock.utcnow()
            >>> now.format('%Y-%m-%d %H:%M:%S %:z')
            '2022-03-23 13:25:50 +00:00'
            >>> now.format('%s')
            '1648041950'
            >>> now.format()
            2022-03-23 13:25:50+00:00'
        """
    def __format__(self, __format_spec: str) -> str: ...
    @property
    def tzinfo(self) -> Tz:
        """Gets the ``atomic_clock.Tz`` of the :class:`AtomicClock <atomic_clock.AtomicClock>` object.

        Usage::
            >>> now = AtomicClock.now('Asia/Shanghai')
            >>> now.tzinfo
            <Tz [Asia/Shanghai]>
        """
    @property
    def datetime(self) -> dt.datetime:
        """Returns a datetime representation of the :class:`AtomicClock <atomic_clock.AtomicClock>` object.

        Usage::
            >>> now = AtomicClock.now()
            >>> now.datetime
            datetime.datetime(2022, 3, 23, 12, 47, 36, 671398, tzinfo=<Tz [local]>)
        """
    @property
    def naive(self) -> dt.datetime:
        """Returns a naive datetime representation of the :class:`AtomicClock <atomic_clock.AtomicClock>`
        object.

        Usage::
            >>> shanghai = AtomicClock.now('Asia/Shanghai')
            >>> shanghai
            <AtomicClock [2022-03-22T13:54:13.294995+08:00]>
            >>> shanghai.naive
            datetime.datetime(2022, 3, 22, 5, 54, 13, 294995)
        """
    def timestamp(self) -> float:
        """Returns a timestamp representation of the :class:`AtomicClock <atomic_clock.AtomiClock>`
        object, in UTC time.

        Usage::
            >>> AtomicClock.utcnow().timestamp()
            1647924832.531622
        """
    @property
    def int_timestamp(self) -> int:
        """Returns an integer timestamp representation of the :class:`AtomicClock <atomic_clock.AtomiClock>`
        object, in UTC time.

        Usage::
            >>> AtomicClock.utcnow().int_timestamp
            1647928543
        """
    @property
    def float_timestamp(self) -> float:
        """Returns a float timestamp representation of the :class:`AtomicClock <atomic_clock.AtomiClock>`
        object, in UTC time.

        Usage::
            >>> AtomicClock.utcnow().float_timestamp
            1647928591.746371
        """

class Tz(dt.tzinfo):
    """A :class: `Tz <atomic_clock.Tz>` object

    Implements the `tzinfo` interface

    :param tzinfo:  A timezone expression.

    .. _tz-expr:

    Recognized timezone expressions:
        - A ``str`` describing a timezone, similar to 'US/Pacific', or 'Asia/Shanghai'.
        - A ``str`` in ISO 8601 style, as in '+07:00'.
        - A ``str``, one of the following:  'local', 'utc', 'UTC'.
    """

    def __init__(self, tzinfo: str) -> None: ...
