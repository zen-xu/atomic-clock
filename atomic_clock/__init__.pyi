from __future__ import annotations

import datetime as dt

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
    def now(cls, tzinfo: str | dt.tzinfo = "local") -> AtomicClock:
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
        tzinfo: str | dt.tzinfo = "local",
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
        cls, dt: dt.datetime, tzinfo: str | dt.tzinfo = "utc"
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
    def fromdate(cls, dt: dt.date, tzinfo: str | dt.tzinfo = "utc") -> AtomicClock:
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
