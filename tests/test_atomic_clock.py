import json
import time

from datetime import date
from datetime import datetime
from datetime import timedelta
from decimal import Decimal

import atomic_clock
import pytest

from dateutil import tz

from .utils import assert_datetime_equality


class TestAtomicClockInit:
    def test_init_bad_input(self):

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock(2013)

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock(2013, 2)

        with pytest.raises(ValueError):
            atomic_clock.AtomicClock(2013, 2, 2, 12, 30, 45, 9999999)

    @pytest.mark.parametrize(
        ["atomic_clock", "datetime"],
        (
            (
                atomic_clock.AtomicClock(2013, 2, 2),
                datetime(2013, 2, 2, tzinfo=tz.tzutc()),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 2, 12),
                datetime(2013, 2, 2, 12, tzinfo=tz.tzutc()),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 2, 12, 30),
                datetime(2013, 2, 2, 12, 30, tzinfo=tz.tzutc()),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 2, 12, 30, 45, 999999),
                datetime(2013, 2, 2, 12, 30, 45, 999999, tzinfo=tz.tzutc()),
            ),
            (
                atomic_clock.AtomicClock(
                    2013, 2, 2, 12, 30, 45, 999999, tzinfo=tz.gettz("Europe/Paris")
                ),
                datetime(
                    2013, 2, 2, 12, 30, 45, 999999, tzinfo=tz.gettz("Europe/Paris")
                ),
            ),
        ),
    )
    def test_init(self, atomic_clock, datetime):
        assert atomic_clock == datetime


class TestTestArrowFactory:
    def test_now(self):

        result = atomic_clock.now()

        assert_datetime_equality(result, datetime.now().replace(tzinfo=tz.tzlocal()))

    def test_utcnow(self):

        result = atomic_clock.utcnow()

        assert_datetime_equality(result, datetime.utcnow().replace(tzinfo=tz.tzutc()))

    def test_fromtimestamp(self):

        timestamp = time.time()

        result = atomic_clock.AtomicClock.fromtimestamp(timestamp)
        assert_datetime_equality(result, datetime.now().replace(tzinfo=tz.tzlocal()))

        # TODO: fix it
        # result = atomic_clock.AtomicClock.fromtimestamp(
        #     timestamp, tzinfo="Europe/Paris"
        # )
        # assert_datetime_equality(
        #     result,
        #     datetime.fromtimestamp(timestamp, tz.gettz("Europe/Paris")),
        # )

        # result = atomic_clock.AtomicClock.fromtimestamp(
        #     timestamp, tzinfo=tz.gettz("Europe/Paris")
        # )
        # assert_datetime_equality(
        #     result,
        #     datetime.fromtimestamp(timestamp, tz.gettz("Europe/Paris")),
        # )

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock.fromtimestamp("invalid timestamp")

    def test_utcfromtimestamp(self):

        timestamp = time.time()

        result = atomic_clock.AtomicClock.utcfromtimestamp(timestamp)
        assert_datetime_equality(result, datetime.utcnow().replace(tzinfo=tz.tzutc()))

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock.utcfromtimestamp("invalid timestamp")

    def test_fromdatetime(self):

        dt = datetime(2013, 2, 3, 12, 30, 45, 1)

        result = atomic_clock.AtomicClock.fromdatetime(dt)

        assert result == dt

    def test_fromdatetime_dt_tzinfo(self):

        dt = datetime(2013, 2, 3, 12, 30, 45, 1, tzinfo=tz.gettz("US/Pacific"))

        result = atomic_clock.AtomicClock.fromdatetime(dt)

        assert result == dt
        assert result.tzinfo.utcoffset(dt) == dt.utcoffset()

    def test_fromdatetime_tzinfo_arg(self):

        dt = datetime(2013, 2, 3, 12, 30, 45, 1)

        result = atomic_clock.AtomicClock.fromdatetime(dt, tz.gettz("US/Pacific"))
        dt = dt.replace(tzinfo=tz.gettz("US/Pacific"))
        assert result == dt
        assert result.tzinfo.utcoffset(dt) == dt.utcoffset()

    def test_fromdate(self):

        dt = date(2013, 2, 3)

        result = atomic_clock.AtomicClock.fromdate(dt, tz.gettz("US/Pacific"))
        dt = datetime(2013, 2, 3, tzinfo=tz.gettz("US/Pacific"))

        assert result == dt
        assert result.tzinfo.utcoffset(dt) == dt.utcoffset()

    def test_strptime(self):

        formatted = datetime(2013, 2, 3, 12, 30, 45).strftime("%Y-%m-%d %H:%M:%S")

        result = atomic_clock.AtomicClock.strptime(formatted, "%Y-%m-%d %H:%M:%S")
        assert result == datetime(2013, 2, 3, 12, 30, 45, tzinfo=tz.tzutc())
        assert result.tzinfo == atomic_clock.Tz("UTC")

        result = atomic_clock.AtomicClock.strptime(
            formatted, "%Y-%m-%d %H:%M:%S", tzinfo="Europe/Paris"
        )
        assert result.tzinfo == atomic_clock.Tz("Europe/Paris")

    def test_fromordinal(self):

        timestamp = 1607066909.937968
        with pytest.raises(TypeError):
            atomic_clock.AtomicClock.fromordinal(timestamp)
        with pytest.raises(ValueError):
            atomic_clock.AtomicClock.fromordinal(int(timestamp))

        ordinal = atomic_clock.utcnow().toordinal()

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock.fromordinal(str(ordinal))

        result = atomic_clock.AtomicClock.fromordinal(ordinal)
        dt = datetime.fromordinal(ordinal)

        assert result.naive == dt


@pytest.mark.usefixtures("time_2013_02_03")
class TestAtomicClockRepresentation:
    def test_repr(self):

        result = self.atomic_clock.__repr__()

        assert result == f"<AtomicClock [{self.atomic_clock.isoformat()}]>"

    def test_str(self):

        result = self.atomic_clock.__str__()

        assert result == self.atomic_clock.isoformat()

    def test_hash(self):

        result = self.atomic_clock.__hash__()

        assert result == Decimal(str(self.atomic_clock.timestamp())) * 1_000_000_000

    def test_format(self):

        result = f"{self.atomic_clock:%Y-%m-%d}"

        assert result == "2013-02-03"

    def test_bare_format(self):

        result = self.atomic_clock.format()

        assert result == "2013-02-03 12:30:45+00:00"

    def test_format_no_format_string(self):

        result = f"{self.atomic_clock}"

        assert result == str(self.atomic_clock)

    def test_clone(self):

        result = self.atomic_clock.clone()

        assert result is not self.atomic_clock
        assert result == self.atomic_clock


@pytest.mark.usefixtures("time_2013_01_01")
class TestAtomicClockAttribute:
    def test_getattr_base(self):

        with pytest.raises(AttributeError):
            self.atomic_clock.prop

    def test_getattr_week(self):

        assert self.atomic_clock.week == 1

    def test_getattr_quarter(self):
        # start dates
        assert atomic_clock.AtomicClock(2013, 1, 1).quarter == 1
        assert atomic_clock.AtomicClock(2013, 4, 1).quarter == 2
        assert atomic_clock.AtomicClock(2013, 8, 1).quarter == 3
        assert atomic_clock.AtomicClock(2013, 10, 1).quarter == 4

        # end dates
        assert atomic_clock.AtomicClock(2013, 3, 31).quarter == 1
        assert atomic_clock.AtomicClock(2013, 6, 30).quarter == 2
        assert atomic_clock.AtomicClock(2013, 9, 30).quarter == 3
        assert atomic_clock.AtomicClock(2013, 12, 31).quarter == 4

    def test_getattr_dt_value(self):

        assert self.atomic_clock.year == 2013

    def test_tzinfo(self):

        assert self.atomic_clock.tzinfo == atomic_clock.Tz("UTC")

    def test_naive(self):

        assert self.atomic_clock.naive == self.atomic_clock.datetime.replace(
            tzinfo=None
        )

    def test_float_timestamp(self):

        assert self.atomic_clock.float_timestamp == self.atomic_clock.timestamp()


@pytest.mark.usefixtures("time_utcnow")
class TestAtomicClockComparison:
    def test_eq(self):

        assert self.atomic_clock == self.now
        assert not (self.atomic_clock == "abc")

    def test_ne(self):

        assert not (self.atomic_clock != self.now)
        assert self.atomic_clock != "abc"

    def test_gt(self):

        arrow_cmp = self.atomic_clock.shift(minutes=1)

        assert not (self.atomic_clock > self.now)

        with pytest.raises(TypeError):
            self.atomic_clock > "abc"  # noqa: B015

        assert self.atomic_clock < arrow_cmp

    def test_ge(self):

        with pytest.raises(TypeError):
            self.atomic_clock >= "abc"  # noqa: B015

        assert self.atomic_clock >= self.atomic_clock

    def test_lt(self):

        arrow_cmp = self.atomic_clock.shift(minutes=1)

        assert not (self.atomic_clock < self.atomic_clock)

        with pytest.raises(TypeError):
            self.atomic_clock < "abc"  # noqa: B015

        assert self.atomic_clock < arrow_cmp

    def test_le(self):

        with pytest.raises(TypeError):
            self.atomic_clock <= "abc"  # noqa: B015

        assert self.atomic_clock <= self.atomic_clock


@pytest.mark.usefixtures("time_2013_01_01")
class TestAtomicClockMath:
    def test_add_timedelta(self):

        result = self.atomic_clock.__add__(timedelta(days=1))

        assert result == datetime(2013, 1, 2, tzinfo=tz.tzutc())

    def test_add_other(self):

        with pytest.raises(TypeError):
            self.atomic_clock + 1

    def test_radd(self):

        result = self.atomic_clock.__radd__(timedelta(days=1))

        assert result == datetime(2013, 1, 2, tzinfo=tz.tzutc())

    def test_sub_timedelta(self):

        result = self.atomic_clock.__sub__(timedelta(days=1))

        assert result == datetime(2012, 12, 31, tzinfo=tz.tzutc())

    def test_sub_datetime(self):

        result = self.atomic_clock.__sub__(datetime(2012, 12, 21, tzinfo=tz.tzutc()))

        assert result == timedelta(days=11)

    def test_sub_arrow(self):

        result = self.atomic_clock.__sub__(
            atomic_clock.AtomicClock(2012, 12, 21, tzinfo=tz.tzutc())
        )

        assert result == timedelta(days=11)

    def test_sub_other(self):

        with pytest.raises(TypeError):
            self.atomic_clock - object()

    def test_rsub_datetime(self):

        result = self.atomic_clock.__rsub__(datetime(2012, 12, 21, tzinfo=tz.tzutc()))

        assert result == timedelta(days=-11)

    def test_rsub_other(self):

        with pytest.raises(TypeError):
            timedelta(days=1) - self.atomic_clock


@pytest.mark.usefixtures("time_utcnow")
class TestAtomicClockDatetimeInterface:
    def test_date(self):

        result = self.atomic_clock.date()

        assert result == self.now.date()

    def test_time(self):

        result = self.atomic_clock.time()
        assert result.hour == self.now.hour
        assert result.minute == self.now.minute
        assert result.second == self.now.second
        assert result.microsecond == self.now.microsecond

    def test_timetz(self):

        result = self.atomic_clock.timez()

        assert result.hour == self.atomic_clock.hour
        assert result.minute == self.atomic_clock.minute
        assert result.second == self.atomic_clock.second
        assert result.microsecond == self.atomic_clock.microsecond
        assert result.tzinfo == atomic_clock.Tz("UTC")

    # def test_astimezone(self):

    #     other_tz = tz.gettz("US/Pacific")

    #     result = self.atomic_clock.astimezone(other_tz)

    #     assert result == self.now.astimezone(other_tz)

    def test_utcoffset(self):

        result = self.atomic_clock.utcoffset()

        assert result == timedelta(0)

    def test_dst(self):

        result = self.atomic_clock.dst()

        assert result == timedelta(0)

    def test_timetuple(self):

        result = self.atomic_clock.timetuple()

        assert result == self.now.timetuple()

    def test_utctimetuple(self):

        result = self.atomic_clock.utctimetuple()

        assert result == self.now.timetuple()

    def test_toordinal(self):

        result = self.atomic_clock.toordinal()

        assert result == self.now.toordinal()

    def test_weekday(self):

        result = self.atomic_clock.weekday()

        assert result == self.now.weekday()

    def test_isoweekday(self):

        result = self.atomic_clock.isoweekday()

        assert result == self.now.isoweekday()

    def test_isocalendar(self):

        result = self.atomic_clock.isocalendar()

        assert list(result) == list(self.now.isocalendar())

    def test_isoformat(self):

        result = self.atomic_clock.isoformat()

        assert result == self.now.isoformat()

    def test_isoformat_timespec(self):

        result = self.atomic_clock.isoformat(timespec="hours")
        assert result == self.now.isoformat(timespec="hours")

        result = self.atomic_clock.isoformat(timespec="microseconds")
        assert result == self.now.isoformat()

        result = self.atomic_clock.isoformat(timespec="milliseconds")
        assert result == self.now.isoformat(timespec="milliseconds")

        result = self.atomic_clock.isoformat(sep="x", timespec="seconds")
        assert result == self.atomic_clock.isoformat(sep="x", timespec="seconds")

    def test_simplejson(self):

        result = json.dumps({"v": self.atomic_clock.for_json()})

        assert json.loads(result)["v"] == self.atomic_clock.isoformat()

    def test_ctime(self):

        result = self.atomic_clock.ctime()

        assert result == self.now.ctime()

    def test_strftime(self):

        result = self.atomic_clock.strftime("%Y")

        assert result == self.now.strftime("%Y")


class TestAtomicClockFalsePositiveDst:
    def test_dst(self):
        self.before_1 = atomic_clock.AtomicClock(
            2016, 11, 6, 3, 59, tzinfo=tz.gettz("America/New_York")
        )
        self.before_2 = atomic_clock.AtomicClock(
            2016, 11, 6, tzinfo=tz.gettz("America/New_York")
        )
        self.after_1 = atomic_clock.AtomicClock(
            2016, 11, 6, 4, tzinfo=tz.gettz("America/New_York")
        )
        self.after_2 = atomic_clock.AtomicClock(
            2016, 11, 6, 23, 59, tzinfo=tz.gettz("America/New_York")
        )
        self.before_3 = atomic_clock.AtomicClock(
            2018, 11, 4, 3, 59, tzinfo=tz.gettz("America/New_York")
        )
        self.before_4 = atomic_clock.AtomicClock(
            2018, 11, 4, tzinfo=tz.gettz("America/New_York")
        )
        self.after_3 = atomic_clock.AtomicClock(
            2018, 11, 4, 4, tzinfo=tz.gettz("America/New_York")
        )
        self.after_4 = atomic_clock.AtomicClock(
            2018, 11, 4, 23, 59, tzinfo=tz.gettz("America/New_York")
        )
        assert self.before_1.day == self.before_2.day
        assert self.after_1.day == self.after_2.day
        assert self.before_3.day == self.before_4.day
        assert self.after_3.day == self.after_4.day


class TestAtomicClockConversion:
    def test_to(self):

        dt_from = datetime.now()
        atomic_clock_from = atomic_clock.AtomicClock.fromdatetime(dt_from, "US/Pacific")
        expected = dt_from.replace(tzinfo=tz.gettz("US/Pacific")).astimezone(tz.tzutc())

        assert atomic_clock_from.to("UTC").isoformat() == expected.isoformat()

    def test_to_pacific_then_utc(self):
        result = (
            atomic_clock.AtomicClock(2018, 11, 4, 1, tzinfo="-08:00")
            .to("US/Pacific")
            .to("UTC")
        )
        assert result == atomic_clock.AtomicClock(2018, 11, 4, 9)

    def test_to_amsterdam_then_utc(self):
        result = atomic_clock.AtomicClock(2016, 10, 30).to("Europe/Amsterdam")
        assert result.utcoffset() == timedelta(seconds=7200)

    # def test_to_israel_same_offset(self):

    #     result = atomic_clock.AtomicClock(2019, 10, 27, 2, 21, 1, tzinfo="+03:00").to(
    #         "Israel"
    #     )
    #     expected = atomic_clock.AtomicClock(2019, 10, 27, 1, 21, 1, tzinfo="Israel")

    #     assert result == expected
    #     assert result.utcoffset() != expected.utcoffset()

    # def test_anchorage_dst(self):
    #     before = atomic_clock.AtomicClock(
    #         2016, 3, 13, 1, 59, tzinfo="America/Anchorage"
    #     )
    #     after = atomic_clock.AtomicClock(2016, 3, 13, 2, 1, tzinfo="America/Anchorage")

    #     assert before.utcoffset() != after.utcoffset()

    # def test_chicago_fall(self):

    #     result = atomic_clock.AtomicClock(2017, 11, 5, 2, 1, tzinfo="-05:00").to(
    #         "America/Chicago"
    #     )
    #     expected = atomic_clock.AtomicClock(2017, 11, 5, 1, 1, tzinfo="America/Chicago")

    #     assert result == expected
    #     assert result.utcoffset() != expected.utcoffset()

    # def test_toronto_gap(self):

    #     before = atomic_clock.AtomicClock(2011, 3, 13, 6, 30, tzinfo="UTC").to(
    #         "America/Toronto"
    #     )
    #     after = atomic_clock.AtomicClock(2011, 3, 13, 7, 30, tzinfo="UTC").to(
    #         "America/Toronto"
    #     )

    #     assert before.datetime.replace(tzinfo=None) == datetime(2011, 3, 13, 1, 30)
    #     assert after.datetime.replace(tzinfo=None) == datetime(2011, 3, 13, 3, 30)

    #     assert before.utcoffset() != after.utcoffset()

    # def test_sydney_gap(self):

    #     before = atomic_clock.AtomicClock(2012, 10, 6, 15, 30, tzinfo="UTC").to(
    #         "Australia/Sydney"
    #     )
    #     after = atomic_clock.AtomicClock(2012, 10, 6, 16, 30, tzinfo="UTC").to(
    #         "Australia/Sydney"
    #     )

    #     assert before.datetime.replace(tzinfo=None) == datetime(2012, 10, 7, 1, 30)
    #     assert after.datetime.replace(tzinfo=None) == datetime(2012, 10, 7, 3, 30)

    #     assert before.utcoffset() != after.utcoffset()


# class TestAtomicClockPickling:
#     def test_pickle_and_unpickle(self):

#         dt = atomic_clock.AtomicClock.utcnow()

#         pickled = pickle.dumps(dt)

#         unpickled = pickle.loads(pickled)

#         assert unpickled == dt


class TestAtomicClockReplace:
    def test_not_attr(self):

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock.utcnow().replace(abc=1)

    def test_replace(self):

        arw = atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 45)

        assert arw.replace(year=2012) == atomic_clock.AtomicClock(
            2012, 5, 5, 12, 30, 45
        )
        assert arw.replace(month=1) == atomic_clock.AtomicClock(2013, 1, 5, 12, 30, 45)
        assert arw.replace(day=1) == atomic_clock.AtomicClock(2013, 5, 1, 12, 30, 45)
        assert arw.replace(hour=1) == atomic_clock.AtomicClock(2013, 5, 5, 1, 30, 45)
        assert arw.replace(minute=1) == atomic_clock.AtomicClock(2013, 5, 5, 12, 1, 45)
        assert arw.replace(second=1) == atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 1)

    def test_replace_tzinfo(self):

        ac = atomic_clock.AtomicClock(2022, 4, 5, 10, 1, 2).to("US/Eastern")

        assert ac.replace(tzinfo="US/Pacific") == atomic_clock.AtomicClock(
            2022, 4, 5, 6, 1, 2, tzinfo="US/Pacific"
        )

    #     def test_replace_fold(self):

    #         before = atomic_clock.AtomicClock(2017, 11, 5, 1, tzinfo="America/New_York")
    #         after = before.replace(fold=1)

    #         assert before.fold == 0
    #         assert after.fold == 1
    #         assert before == after
    #         assert before.utcoffset() != after.utcoffset()

    #     def test_replace_fold_and_other(self):

    #         arw = atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 45)

    #         assert arw.replace(fold=1, minute=50) == atomic_clock.AtomicClock(2013, 5, 5, 12, 50, 45)
    #         assert arw.replace(minute=50, fold=1) == atomic_clock.AtomicClock(2013, 5, 5, 12, 50, 45)

    def test_replace_week(self):

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock.utcnow().replace(week=1)

    def test_replace_quarter(self):

        with pytest.raises(TypeError):
            atomic_clock.AtomicClock.utcnow().replace(quarter=1)

    #     def test_replace_quarter_and_fold(self):
    #         with pytest.raises(AttributeError):
    #             atomic_clock.utcnow().replace(fold=1, quarter=1)

    #         with pytest.raises(AttributeError):
    #             atomic_clock.utcnow().replace(quarter=1, fold=1)

    def test_replace_other_kwargs(self):

        with pytest.raises(TypeError):
            atomic_clock.utcnow().replace(abc="def")


class TestAtomicClockShift:
    def test_not_attr(self):

        now = atomic_clock.AtomicClock.utcnow()

        with pytest.raises(TypeError):
            now.shift(abc=1)

        with pytest.raises(TypeError):
            now.shift(week=1)

    def test_shift(self):

        ac = atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 45)

        assert ac.shift(years=1) == atomic_clock.AtomicClock(2014, 5, 5, 12, 30, 45)
        assert ac.shift(quarters=1) == atomic_clock.AtomicClock(2013, 8, 5, 12, 30, 45)
        assert ac.shift(quarters=1, months=1) == atomic_clock.AtomicClock(
            2013, 9, 5, 12, 30, 45
        )
        assert ac.shift(months=1) == atomic_clock.AtomicClock(2013, 6, 5, 12, 30, 45)
        assert ac.shift(weeks=1) == atomic_clock.AtomicClock(2013, 5, 12, 12, 30, 45)
        assert ac.shift(days=1) == atomic_clock.AtomicClock(2013, 5, 6, 12, 30, 45)
        assert ac.shift(hours=1) == atomic_clock.AtomicClock(2013, 5, 5, 13, 30, 45)
        assert ac.shift(minutes=1) == atomic_clock.AtomicClock(2013, 5, 5, 12, 31, 45)
        assert ac.shift(seconds=1) == atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 46)
        assert ac.shift(microseconds=1) == atomic_clock.AtomicClock(
            2013, 5, 5, 12, 30, 45, 1
        )

        # # Remember: Python's weekday 0 is Monday
        assert ac.shift(weekday=0) == atomic_clock.AtomicClock(2013, 5, 6, 12, 30, 45)
        assert ac.shift(weekday=1) == atomic_clock.AtomicClock(2013, 5, 7, 12, 30, 45)
        assert ac.shift(weekday=2) == atomic_clock.AtomicClock(2013, 5, 8, 12, 30, 45)
        assert ac.shift(weekday=3) == atomic_clock.AtomicClock(2013, 5, 9, 12, 30, 45)
        assert ac.shift(weekday=4) == atomic_clock.AtomicClock(2013, 5, 10, 12, 30, 45)
        assert ac.shift(weekday=5) == atomic_clock.AtomicClock(2013, 5, 11, 12, 30, 45)
        assert ac.shift(weekday=6) == ac

        with pytest.raises(IndexError):
            ac.shift(weekday=7)

        # Use dateutil.relativedelta's convenient day instances
        # assert arw.shift(weekday=MO) == atomic_clock.AtomicClock(2013, 5, 6, 12, 30, 45)
        # assert arw.shift(weekday=MO(0)) == atomic_clock.AtomicClock(
        #     2013, 5, 6, 12, 30, 45
        # )
        # assert arw.shift(weekday=MO(1)) == atomic_clock.AtomicClock(
        #     2013, 5, 6, 12, 30, 45
        # )
        # assert arw.shift(weekday=MO(2)) == atomic_clock.AtomicClock(
        #     2013, 5, 13, 12, 30, 45
        # )
        # assert arw.shift(weekday=TU) == atomic_clock.AtomicClock(2013, 5, 7, 12, 30, 45)
        # assert arw.shift(weekday=TU(0)) == atomic_clock.AtomicClock(
        #     2013, 5, 7, 12, 30, 45
        # )
        # assert arw.shift(weekday=TU(1)) == atomic_clock.AtomicClock(
        #     2013, 5, 7, 12, 30, 45
        # )
        # assert arw.shift(weekday=TU(2)) == atomic_clock.AtomicClock(
        #     2013, 5, 14, 12, 30, 45
        # )
        # assert arw.shift(weekday=WE) == atomic_clock.AtomicClock(2013, 5, 8, 12, 30, 45)
        # assert arw.shift(weekday=WE(0)) == atomic_clock.AtomicClock(
        #     2013, 5, 8, 12, 30, 45
        # )
        # assert arw.shift(weekday=WE(1)) == atomic_clock.AtomicClock(
        #     2013, 5, 8, 12, 30, 45
        # )
        # assert arw.shift(weekday=WE(2)) == atomic_clock.AtomicClock(
        #     2013, 5, 15, 12, 30, 45
        # )
        # assert arw.shift(weekday=TH) == atomic_clock.AtomicClock(2013, 5, 9, 12, 30, 45)
        # assert arw.shift(weekday=TH(0)) == atomic_clock.AtomicClock(
        #     2013, 5, 9, 12, 30, 45
        # )
        # assert arw.shift(weekday=TH(1)) == atomic_clock.AtomicClock(
        #     2013, 5, 9, 12, 30, 45
        # )
        # assert arw.shift(weekday=TH(2)) == atomic_clock.AtomicClock(
        #     2013, 5, 16, 12, 30, 45
        # )
        # assert arw.shift(weekday=FR) == atomic_clock.AtomicClock(
        #     2013, 5, 10, 12, 30, 45
        # )
        # assert arw.shift(weekday=FR(0)) == atomic_clock.AtomicClock(
        #     2013, 5, 10, 12, 30, 45
        # )
        # assert arw.shift(weekday=FR(1)) == atomic_clock.AtomicClock(
        #     2013, 5, 10, 12, 30, 45
        # )
        # assert arw.shift(weekday=FR(2)) == atomic_clock.AtomicClock(
        #     2013, 5, 17, 12, 30, 45
        # )
        # assert arw.shift(weekday=SA) == atomic_clock.AtomicClock(
        #     2013, 5, 11, 12, 30, 45
        # )
        # assert arw.shift(weekday=SA(0)) == atomic_clock.AtomicClock(
        #     2013, 5, 11, 12, 30, 45
        # )
        # assert arw.shift(weekday=SA(1)) == atomic_clock.AtomicClock(
        #     2013, 5, 11, 12, 30, 45
        # )
        # assert arw.shift(weekday=SA(2)) == atomic_clock.AtomicClock(
        #     2013, 5, 18, 12, 30, 45
        # )
        # assert arw.shift(weekday=SU) == arw
        # assert arw.shift(weekday=SU(0)) == arw
        # assert arw.shift(weekday=SU(1)) == arw
        # assert arw.shift(weekday=SU(2)) == atomic_clock.AtomicClock(
        #     2013, 5, 12, 12, 30, 45
        # )

    def test_shift_negative(self):

        ac = atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 45)

        assert ac.shift(years=-1) == atomic_clock.AtomicClock(2012, 5, 5, 12, 30, 45)
        assert ac.shift(quarters=-1) == atomic_clock.AtomicClock(2013, 2, 5, 12, 30, 45)
        assert ac.shift(quarters=-1, months=-1) == atomic_clock.AtomicClock(
            2013, 1, 5, 12, 30, 45
        )
        assert ac.shift(months=-1) == atomic_clock.AtomicClock(2013, 4, 5, 12, 30, 45)
        assert ac.shift(weeks=-1) == atomic_clock.AtomicClock(2013, 4, 28, 12, 30, 45)
        assert ac.shift(days=-1) == atomic_clock.AtomicClock(2013, 5, 4, 12, 30, 45)
        assert ac.shift(hours=-1) == atomic_clock.AtomicClock(2013, 5, 5, 11, 30, 45)
        assert ac.shift(minutes=-1) == atomic_clock.AtomicClock(2013, 5, 5, 12, 29, 45)
        assert ac.shift(seconds=-1) == atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 44)
        assert ac.shift(microseconds=-1) == atomic_clock.AtomicClock(
            2013, 5, 5, 12, 30, 44, 999999
        )

        with pytest.raises(IndexError):
            ac.shift(weekday=-8)

        # assert arw.shift(weekday=MO(-1)) == atomic_clock.AtomicClock(
        #     2013, 4, 29, 12, 30, 45
        # )
        # assert arw.shift(weekday=TU(-1)) == atomic_clock.AtomicClock(
        #     2013, 4, 30, 12, 30, 45
        # )
        # assert arw.shift(weekday=WE(-1)) == atomic_clock.AtomicClock(
        #     2013, 5, 1, 12, 30, 45
        # )
        # assert arw.shift(weekday=TH(-1)) == atomic_clock.AtomicClock(
        #     2013, 5, 2, 12, 30, 45
        # )
        # assert arw.shift(weekday=FR(-1)) == atomic_clock.AtomicClock(
        #     2013, 5, 3, 12, 30, 45
        # )
        # assert arw.shift(weekday=SA(-1)) == atomic_clock.AtomicClock(
        #     2013, 5, 4, 12, 30, 45
        # )
        # assert arw.shift(weekday=SU(-1)) == arw
        # assert arw.shift(weekday=SU(-2)) == atomic_clock.AtomicClock(
        #     2013, 4, 28, 12, 30, 45
        # )

    def test_shift_quarters_bug(self):

        ac = atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 45)

        # The value of the last-read argument was used instead of the ``quarters`` argument.
        # Recall that the keyword argument dict, like all dicts, is unordered, so only certain
        # combinations of arguments would exhibit this.
        assert ac.shift(quarters=0, years=1) == atomic_clock.AtomicClock(
            2014, 5, 5, 12, 30, 45
        )
        assert ac.shift(quarters=0, months=1) == atomic_clock.AtomicClock(
            2013, 6, 5, 12, 30, 45
        )
        assert ac.shift(quarters=0, weeks=1) == atomic_clock.AtomicClock(
            2013, 5, 12, 12, 30, 45
        )
        assert ac.shift(quarters=0, days=1) == atomic_clock.AtomicClock(
            2013, 5, 6, 12, 30, 45
        )
        assert ac.shift(quarters=0, hours=1) == atomic_clock.AtomicClock(
            2013, 5, 5, 13, 30, 45
        )
        assert ac.shift(quarters=0, minutes=1) == atomic_clock.AtomicClock(
            2013, 5, 5, 12, 31, 45
        )
        assert ac.shift(quarters=0, seconds=1) == atomic_clock.AtomicClock(
            2013, 5, 5, 12, 30, 46
        )
        assert ac.shift(quarters=0, microseconds=1) == atomic_clock.AtomicClock(
            2013, 5, 5, 12, 30, 45, 1
        )

    def test_shift_positive_imaginary(self):

        # Avoid shifting into imaginary datetimes, take into account DST and other timezone changes.

        new_york = atomic_clock.AtomicClock(
            2017, 3, 12, 1, 30, tzinfo="America/New_York"
        )
        assert new_york.shift(hours=+1) == atomic_clock.AtomicClock(
            2017, 3, 12, 3, 30, tzinfo="America/New_York"
        )

        # pendulum example
        paris = atomic_clock.AtomicClock(2013, 3, 31, 1, 50, tzinfo="Europe/Paris")
        assert paris.shift(minutes=+20) == atomic_clock.AtomicClock(
            2013, 3, 31, 3, 10, tzinfo="Europe/Paris"
        )

        canberra = atomic_clock.AtomicClock(
            2018, 10, 7, 1, 30, tzinfo="Australia/Canberra"
        )
        assert canberra.shift(hours=+1) == atomic_clock.AtomicClock(
            2018, 10, 7, 3, 30, tzinfo="Australia/Canberra"
        )

        kiev = atomic_clock.AtomicClock(2018, 3, 25, 2, 30, tzinfo="Europe/Kiev")
        assert kiev.shift(hours=+1) == atomic_clock.AtomicClock(
            2018, 3, 25, 4, 30, tzinfo="Europe/Kiev"
        )

        # Edge case, the entire day of 2011-12-30 is imaginary in this zone!
        apia = atomic_clock.AtomicClock(2011, 12, 29, 23, tzinfo="Pacific/Apia")
        assert apia.shift(hours=+2) == atomic_clock.AtomicClock(
            2011, 12, 31, 1, tzinfo="Pacific/Apia"
        )

    # def test_shift_negative_imaginary(self):

    #     new_york = atomic_clock.AtomicClock(
    #         2011, 3, 13, 3, 30, tzinfo="America/New_York"
    #     )
    #     assert new_york.shift(hours=-1) == atomic_clock.AtomicClock(
    #         2011, 3, 13, 3, 30, tzinfo="America/New_York"
    #     )
    #     assert new_york.shift(hours=-2) == atomic_clock.AtomicClock(
    #         2011, 3, 13, 1, 30, tzinfo="America/New_York"
    #     )

    #     london = atomic_clock.AtomicClock(2019, 3, 31, 2, tzinfo="Europe/London")
    #     assert london.shift(hours=-1) == atomic_clock.AtomicClock(
    #         2019, 3, 31, 2, tzinfo="Europe/London"
    #     )
    #     assert london.shift(hours=-2) == atomic_clock.AtomicClock(
    #         2019, 3, 31, 0, tzinfo="Europe/London"
    #     )

    #     # edge case, crossing the international dateline
    #     apia = atomic_clock.AtomicClock(2011, 12, 31, 1, tzinfo="Pacific/Apia")
    #     assert apia.shift(hours=-2) == atomic_clock.AtomicClock(
    #         2011, 12, 31, 23, tzinfo="Pacific/Apia"
    #     )

    def test_shift_kiritimati(self):
        # corrected 2018d tz database release, will fail in earlier versions

        kiritimati = atomic_clock.AtomicClock(
            1994, 12, 30, 12, 30, tzinfo="Pacific/Kiritimati"
        )
        assert kiritimati.shift(days=+1) == atomic_clock.AtomicClock(
            1995, 1, 1, 12, 30, tzinfo="Pacific/Kiritimati"
        )

    def shift_imaginary_seconds(self):
        # offset has a seconds component
        monrovia = atomic_clock.AtomicClock(1972, 1, 6, 23, tzinfo="Africa/Monrovia")
        assert monrovia.shift(hours=+1, minutes=+30) == atomic_clock.AtomicClock(
            1972, 1, 7, 1, 14, 30, tzinfo="Africa/Monrovia"
        )


class TestArrowRange:
    def test_year(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "year", datetime(2013, 1, 2, 3, 4, 5), datetime(2016, 4, 5, 6, 7, 8)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2014, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2015, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2016, 1, 2, 3, 4, 5),
        ]

    def test_quarter(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "quarter", datetime(2013, 2, 3, 4, 5, 6), datetime(2013, 5, 6, 7, 8, 9)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 2, 3, 4, 5, 6),
            atomic_clock.AtomicClock(2013, 5, 3, 4, 5, 6),
        ]

    def test_month(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "month", datetime(2013, 2, 3, 4, 5, 6), datetime(2013, 5, 6, 7, 8, 9)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 2, 3, 4, 5, 6),
            atomic_clock.AtomicClock(2013, 3, 3, 4, 5, 6),
            atomic_clock.AtomicClock(2013, 4, 3, 4, 5, 6),
            atomic_clock.AtomicClock(2013, 5, 3, 4, 5, 6),
        ]

    def test_week(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "week", datetime(2013, 9, 1, 2, 3, 4), datetime(2013, 10, 1, 2, 3, 4)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 9, 1, 2, 3, 4),
            atomic_clock.AtomicClock(2013, 9, 8, 2, 3, 4),
            atomic_clock.AtomicClock(2013, 9, 15, 2, 3, 4),
            atomic_clock.AtomicClock(2013, 9, 22, 2, 3, 4),
            atomic_clock.AtomicClock(2013, 9, 29, 2, 3, 4),
        ]

    def test_day(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "day", datetime(2013, 1, 2, 3, 4, 5), datetime(2013, 1, 5, 6, 7, 8)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 3, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 4, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 5, 3, 4, 5),
        ]

    def test_hour(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "hour", datetime(2013, 1, 2, 3, 4, 5), datetime(2013, 1, 2, 6, 7, 8)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 2, 4, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 2, 5, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 2, 6, 4, 5),
        ]

        result = list(
            atomic_clock.AtomicClock.range(
                "hour", datetime(2013, 1, 2, 3, 4, 5), datetime(2013, 1, 2, 3, 4, 5)
            )
        )

        assert result == [atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5)]

    def test_minute(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "minute", datetime(2013, 1, 2, 3, 4, 5), datetime(2013, 1, 2, 3, 7, 8)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 2, 3, 5, 5),
            atomic_clock.AtomicClock(2013, 1, 2, 3, 6, 5),
            atomic_clock.AtomicClock(2013, 1, 2, 3, 7, 5),
        ]

    def test_second(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "second", datetime(2013, 1, 2, 3, 4, 5), datetime(2013, 1, 2, 3, 4, 8)
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 6),
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 7),
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 8),
        ]

    def test_atomic_clock(self):

        result = list(
            atomic_clock.AtomicClock.range(
                "day",
                atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5),
                atomic_clock.AtomicClock(2013, 1, 5, 6, 7, 8),
            )
        )

        assert result == [
            atomic_clock.AtomicClock(2013, 1, 2, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 3, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 4, 3, 4, 5),
            atomic_clock.AtomicClock(2013, 1, 5, 3, 4, 5),
        ]

    def test_naive_tz(self):

        result = atomic_clock.AtomicClock.range(
            "year",
            datetime(2013, 1, 2, 3),
            datetime(2016, 4, 5, 6),
            tz="US/Pacific",
        )

        for r in result:
            assert r.tzinfo == atomic_clock.Tz("US/Pacific")

    def test_aware_same_tz(self):

        result = atomic_clock.AtomicClock.range(
            "day",
            atomic_clock.AtomicClock(2013, 1, 1, tzinfo=atomic_clock.Tz("US/Pacific")),
            atomic_clock.AtomicClock(2013, 1, 3, tzinfo=atomic_clock.Tz("US/Pacific")),
        )

        for r in result:
            assert r.tzinfo == atomic_clock.Tz("US/Pacific")

    def test_aware_different_tz(self):

        result = atomic_clock.AtomicClock.range(
            "day",
            datetime(2013, 1, 1, tzinfo=atomic_clock.Tz("US/Eastern")),
            datetime(2013, 1, 3, tzinfo=atomic_clock.Tz("US/Pacific")),
        )

        for r in result:
            assert r.tzinfo == atomic_clock.Tz("US/Eastern")

    def test_aware_tz(self):

        result = atomic_clock.AtomicClock.range(
            "day",
            datetime(2013, 1, 1, tzinfo=atomic_clock.Tz("US/Eastern")),
            datetime(2013, 1, 3, tzinfo=atomic_clock.Tz("US/Pacific")),
            tz=atomic_clock.Tz("US/Central"),
        )

        for r in result:
            assert r.tzinfo == atomic_clock.Tz("US/Central")

    def test_imaginary(self):
        before = atomic_clock.AtomicClock(2018, 3, 10, 23, tzinfo="US/Pacific")
        after = atomic_clock.AtomicClock(2018, 3, 11, 4, tzinfo="US/Pacific")

        pacific_range = list(atomic_clock.AtomicClock.range("hour", before, after))

        utc_range = [
            t.to("utc") for t in atomic_clock.AtomicClock.range("hour", before, after)
        ]

        assert len(pacific_range) == len(set(pacific_range))
        assert len(utc_range) == len(set(utc_range))

    def test_unsupported(self):

        with pytest.raises(ValueError):
            next(
                atomic_clock.AtomicClock.range(
                    "abc", datetime.utcnow(), datetime.utcnow()
                )
            )

    def test_range_over_months_ending_on_different_days(self):
        result = list(
            atomic_clock.AtomicClock.range("month", datetime(2015, 1, 31), limit=4)
        )
        assert result == [
            atomic_clock.AtomicClock(2015, 1, 31),
            atomic_clock.AtomicClock(2015, 2, 28),
            atomic_clock.AtomicClock(2015, 3, 31),
            atomic_clock.AtomicClock(2015, 4, 30),
        ]

        result = list(
            atomic_clock.AtomicClock.range("month", datetime(2015, 1, 30), limit=3)
        )
        assert result == [
            atomic_clock.AtomicClock(2015, 1, 30),
            atomic_clock.AtomicClock(2015, 2, 28),
            atomic_clock.AtomicClock(2015, 3, 30),
        ]

        result = list(
            atomic_clock.AtomicClock.range("month", datetime(2015, 2, 28), limit=3)
        )
        assert result == [
            atomic_clock.AtomicClock(2015, 2, 28),
            atomic_clock.AtomicClock(2015, 3, 28),
            atomic_clock.AtomicClock(2015, 4, 28),
        ]

        result = list(
            atomic_clock.AtomicClock.range("month", datetime(2015, 3, 31), limit=3)
        )
        assert result == [
            atomic_clock.AtomicClock(2015, 3, 31),
            atomic_clock.AtomicClock(2015, 4, 30),
            atomic_clock.AtomicClock(2015, 5, 31),
        ]

    def test_range_over_quarter_months_ending_on_different_days(self):
        result = list(
            atomic_clock.AtomicClock.range("quarter", datetime(2014, 11, 30), limit=3)
        )
        assert result == [
            atomic_clock.AtomicClock(2014, 11, 30),
            atomic_clock.AtomicClock(2015, 2, 28),
            atomic_clock.AtomicClock(2015, 5, 30),
        ]

    def test_range_over_year_maintains_end_date_across_leap_year(self):
        result = list(
            atomic_clock.AtomicClock.range("year", datetime(2012, 2, 29), limit=5)
        )
        assert result == [
            atomic_clock.AtomicClock(2012, 2, 29),
            atomic_clock.AtomicClock(2013, 2, 28),
            atomic_clock.AtomicClock(2014, 2, 28),
            atomic_clock.AtomicClock(2015, 2, 28),
            atomic_clock.AtomicClock(2016, 2, 29),
        ]


class TestAtomicClockSpanRange:
    def test_year(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "year", datetime(2013, 2, 1), datetime(2016, 3, 31)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1),
                atomic_clock.AtomicClock(2013, 12, 31, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2014, 1, 1),
                atomic_clock.AtomicClock(2014, 12, 31, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2015, 1, 1),
                atomic_clock.AtomicClock(2015, 12, 31, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2016, 1, 1),
                atomic_clock.AtomicClock(2016, 12, 31, 23, 59, 59, 999999),
            ),
        ]

    def test_quarter(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "quarter", datetime(2013, 2, 2), datetime(2013, 5, 15)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1),
                atomic_clock.AtomicClock(2013, 3, 31, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 4, 1),
                atomic_clock.AtomicClock(2013, 6, 30, 23, 59, 59, 999999),
            ),
        ]

    def test_month(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "month", datetime(2013, 1, 2), datetime(2013, 4, 15)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1),
                atomic_clock.AtomicClock(2013, 1, 31, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 1),
                atomic_clock.AtomicClock(2013, 2, 28, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 3, 1),
                atomic_clock.AtomicClock(2013, 3, 31, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 4, 1),
                atomic_clock.AtomicClock(2013, 4, 30, 23, 59, 59, 999999),
            ),
        ]

    def test_week(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "week", datetime(2013, 2, 2), datetime(2013, 2, 28)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 28),
                atomic_clock.AtomicClock(2013, 2, 3, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 4),
                atomic_clock.AtomicClock(2013, 2, 10, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 11),
                atomic_clock.AtomicClock(2013, 2, 17, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 18),
                atomic_clock.AtomicClock(2013, 2, 24, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 2, 25),
                atomic_clock.AtomicClock(2013, 3, 3, 23, 59, 59, 999999),
            ),
        ]

    def test_day(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "day", datetime(2013, 1, 1, 12), datetime(2013, 1, 4, 12)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0),
                atomic_clock.AtomicClock(2013, 1, 1, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 2, 0),
                atomic_clock.AtomicClock(2013, 1, 2, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 3, 0),
                atomic_clock.AtomicClock(2013, 1, 3, 23, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 4, 0),
                atomic_clock.AtomicClock(2013, 1, 4, 23, 59, 59, 999999),
            ),
        ]

    def test_hour(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "hour", datetime(2013, 1, 1, 0, 30), datetime(2013, 1, 1, 3, 30)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 1),
                atomic_clock.AtomicClock(2013, 1, 1, 1, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 2),
                atomic_clock.AtomicClock(2013, 1, 1, 2, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 3),
                atomic_clock.AtomicClock(2013, 1, 1, 3, 59, 59, 999999),
            ),
        ]

        result = list(
            atomic_clock.AtomicClock.span_range(
                "hour", datetime(2013, 1, 1, 3, 30), datetime(2013, 1, 1, 3, 30)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1, 3),
                atomic_clock.AtomicClock(2013, 1, 1, 3, 59, 59, 999999),
            )
        ]

    def test_minute(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "minute", datetime(2013, 1, 1, 0, 0, 30), datetime(2013, 1, 1, 0, 3, 30)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 1),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 1, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 2),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 2, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 3),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 3, 59, 999999),
            ),
        ]

    def test_second(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "second", datetime(2013, 1, 1), datetime(2013, 1, 1, 0, 0, 3)
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 0),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 0, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 1),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 1, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 2),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 2, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 3),
                atomic_clock.AtomicClock(2013, 1, 1, 0, 0, 3, 999999),
            ),
        ]

    def test_naive_tz(self):

        tzinfo = atomic_clock.Tz("US/Pacific")

        result = atomic_clock.AtomicClock.span_range(
            "hour",
            datetime(2013, 1, 1, 0),
            datetime(2013, 1, 1, 3, 59),
            tz="US/Pacific",
        )

        for f, c in result:
            assert f.tzinfo == tzinfo
            assert c.tzinfo == tzinfo

    def test_aware_same_tz(self):

        tzinfo = atomic_clock.Tz("US/Pacific")

        result = atomic_clock.AtomicClock.span_range(
            "hour",
            datetime(2013, 1, 1, 0, tzinfo=tzinfo),
            datetime(2013, 1, 1, 2, 59, tzinfo=tzinfo),
        )

        for f, c in result:
            assert f.tzinfo == tzinfo
            assert c.tzinfo == tzinfo

    def test_aware_different_tz(self):

        tzinfo1 = atomic_clock.Tz("US/Pacific")
        tzinfo2 = atomic_clock.Tz("US/Eastern")

        result = atomic_clock.AtomicClock.span_range(
            "hour",
            datetime(2013, 1, 1, 0, tzinfo=tzinfo1),
            datetime(2013, 1, 1, 2, 59, tzinfo=tzinfo2),
        )

        for f, c in result:
            assert f.tzinfo == tzinfo1
            assert c.tzinfo == tzinfo1

    def test_aware_tz(self):

        result = atomic_clock.AtomicClock.span_range(
            "hour",
            datetime(2013, 1, 1, 0, tzinfo=atomic_clock.Tz("US/Eastern")),
            datetime(2013, 1, 1, 2, 59, tzinfo=atomic_clock.Tz("US/Eastern")),
            tz="US/Central",
        )

        for f, c in result:
            assert f.tzinfo == atomic_clock.Tz("US/Central")
            assert c.tzinfo == atomic_clock.Tz("US/Central")

    def test_bounds_param_is_passed(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "quarter", datetime(2013, 2, 2), datetime(2013, 5, 15), bounds="[]"
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 1, 1),
                atomic_clock.AtomicClock(2013, 4, 1),
            ),
            (
                atomic_clock.AtomicClock(2013, 4, 1),
                atomic_clock.AtomicClock(2013, 7, 1),
            ),
        ]

    def test_exact_bound_exclude(self):

        result = list(
            atomic_clock.AtomicClock.span_range(
                "hour",
                datetime(2013, 5, 5, 12, 30),
                datetime(2013, 5, 5, 17, 15),
                bounds="[)",
                exact=True,
            )
        )

        expected = [
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 13, 29, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 13, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 14, 29, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 14, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 15, 29, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 15, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 16, 29, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 16, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 17, 14, 59, 999999),
            ),
        ]

        assert result == expected

    def test_exact_floor_equals_end(self):
        result = list(
            atomic_clock.AtomicClock.span_range(
                "minute",
                datetime(2013, 5, 5, 12, 30),
                datetime(2013, 5, 5, 12, 40),
                exact=True,
            )
        )

        expected = [
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 30, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 31),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 31, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 32),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 32, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 33),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 33, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 34),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 34, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 35),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 35, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 36),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 36, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 37),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 37, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 38),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 38, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 39),
                atomic_clock.AtomicClock(2013, 5, 5, 12, 39, 59, 999999),
            ),
        ]

        assert result == expected

    def test_exact_bound_include(self):
        result = list(
            atomic_clock.AtomicClock.span_range(
                "hour",
                datetime(2013, 5, 5, 2, 30),
                datetime(2013, 5, 5, 6, 00),
                bounds="(]",
                exact=True,
            )
        )

        expected = [
            (
                atomic_clock.AtomicClock(2013, 5, 5, 2, 30, 00, 1),
                atomic_clock.AtomicClock(2013, 5, 5, 3, 30, 00, 0),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 3, 30, 00, 1),
                atomic_clock.AtomicClock(2013, 5, 5, 4, 30, 00, 0),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 4, 30, 00, 1),
                atomic_clock.AtomicClock(2013, 5, 5, 5, 30, 00, 0),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 5, 30, 00, 1),
                atomic_clock.AtomicClock(2013, 5, 5, 6, 00),
            ),
        ]

        assert result == expected

    def test_small_interval_exact_open_bounds(self):
        result = list(
            atomic_clock.AtomicClock.span_range(
                "minute",
                datetime(2013, 5, 5, 2, 30),
                datetime(2013, 5, 5, 2, 31),
                bounds="()",
                exact=True,
            )
        )

        expected = [
            (
                atomic_clock.AtomicClock(2013, 5, 5, 2, 30, 00, 1),
                atomic_clock.AtomicClock(2013, 5, 5, 2, 30, 59, 999999),
            ),
        ]

        assert result == expected


class TestAtomicClockInterval:
    def test_incorrect_input(self):
        with pytest.raises(ValueError):
            list(
                atomic_clock.AtomicClock.interval(
                    "month", datetime(2013, 1, 2), datetime(2013, 4, 15), interval=0
                )
            )

    def test_correct(self):
        result = list(
            atomic_clock.AtomicClock.interval(
                "hour",
                datetime(2013, 5, 5, 12, 30),
                datetime(2013, 5, 5, 17, 15),
                interval=2,
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12),
                atomic_clock.AtomicClock(2013, 5, 5, 13, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 14),
                atomic_clock.AtomicClock(2013, 5, 5, 15, 59, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 16),
                atomic_clock.AtomicClock(2013, 5, 5, 17, 59, 59, 999999),
            ),
        ]

    def test_bounds_param_is_passed(self):
        result = list(
            atomic_clock.AtomicClock.interval(
                "hour",
                datetime(2013, 5, 5, 12, 30),
                datetime(2013, 5, 5, 17, 15),
                interval=2,
                bounds="[]",
            )
        )

        assert result == [
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12),
                atomic_clock.AtomicClock(2013, 5, 5, 14),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 14),
                atomic_clock.AtomicClock(2013, 5, 5, 16),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 16),
                atomic_clock.AtomicClock(2013, 5, 5, 18),
            ),
        ]

    def test_exact(self):
        result = list(
            atomic_clock.AtomicClock.interval(
                "hour",
                datetime(2013, 5, 5, 12, 30),
                datetime(2013, 5, 5, 17, 15),
                interval=4,
                exact=True,
            )
        )

        expected = [
            (
                atomic_clock.AtomicClock(2013, 5, 5, 12, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 16, 29, 59, 999999),
            ),
            (
                atomic_clock.AtomicClock(2013, 5, 5, 16, 30),
                atomic_clock.AtomicClock(2013, 5, 5, 17, 14, 59, 999999),
            ),
        ]

        assert result == expected


@pytest.mark.usefixtures("time_2013_02_15")
class TestAtomicClockSpan:
    def test_span_attribute(self):

        with pytest.raises(ValueError):
            self.atomic_clock.span("span")

    def test_span_year(self):

        floor, ceil = self.atomic_clock.span("year")

        assert floor == datetime(2013, 1, 1, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 12, 31, 23, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_quarter(self):

        floor, ceil = self.atomic_clock.span("quarter")

        assert floor == datetime(2013, 1, 1, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 3, 31, 23, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_quarter_count(self):

        floor, ceil = self.atomic_clock.span("quarter", count=2)

        assert floor == datetime(2013, 1, 1, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 6, 30, 23, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_year_count(self):

        floor, ceil = self.atomic_clock.span("year", count=2)

        assert floor == datetime(2013, 1, 1, tzinfo=tz.tzutc())
        assert ceil == datetime(2014, 12, 31, 23, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_month(self):

        floor, ceil = self.atomic_clock.span("month")

        assert floor == datetime(2013, 2, 1, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 28, 23, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_week(self):
        """
        >>> self.atomic_clock.format("YYYY-MM-DD") == "2013-02-15"
        >>> self.atomic_clock.isoweekday() == 5  # a Friday
        """
        # span week from Monday to Sunday
        floor, ceil = self.atomic_clock.span("week")

        assert floor == datetime(2013, 2, 11, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 17, 23, 59, 59, 999999, tzinfo=tz.tzutc())
        # span week from Tuesday to Monday
        floor, ceil = self.atomic_clock.span("week", week_start=2)

        assert floor == datetime(2013, 2, 12, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 18, 23, 59, 59, 999999, tzinfo=tz.tzutc())
        # span week from Saturday to Friday
        floor, ceil = self.atomic_clock.span("week", week_start=6)

        assert floor == datetime(2013, 2, 9, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 23, 59, 59, 999999, tzinfo=tz.tzutc())
        # span week from Sunday to Saturday
        floor, ceil = self.atomic_clock.span("week", week_start=7)

        assert floor == datetime(2013, 2, 10, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 16, 23, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_day(self):

        floor, ceil = self.atomic_clock.span("day")

        assert floor == datetime(2013, 2, 15, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 23, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_hour(self):

        floor, ceil = self.atomic_clock.span("hour")

        assert floor == datetime(2013, 2, 15, 3, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 3, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_span_minute(self):

        floor, ceil = self.atomic_clock.span("minute")

        assert floor == datetime(2013, 2, 15, 3, 41, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 3, 41, 59, 999999, tzinfo=tz.tzutc())

    def test_span_second(self):

        floor, ceil = self.atomic_clock.span("second")

        assert floor == datetime(2013, 2, 15, 3, 41, 22, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 3, 41, 22, 999999, tzinfo=tz.tzutc())

    def test_span_microsecond(self):

        with pytest.raises(ValueError):
            floor, ceil = self.atomic_clock.span("microsecond")

    def test_floor(self):

        floor, ceil = self.atomic_clock.span("month")

        assert floor == self.atomic_clock.floor("month")
        assert ceil == self.atomic_clock.ceil("month")

    def test_span_inclusive_inclusive(self):

        floor, ceil = self.atomic_clock.span("hour", bounds="[]")

        assert floor == datetime(2013, 2, 15, 3, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 4, tzinfo=tz.tzutc())

    def test_span_exclusive_inclusive(self):

        floor, ceil = self.atomic_clock.span("hour", bounds="(]")

        assert floor == datetime(2013, 2, 15, 3, 0, 0, 1, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 4, tzinfo=tz.tzutc())

    def test_span_exclusive_exclusive(self):

        floor, ceil = self.atomic_clock.span("hour", bounds="()")

        assert floor == datetime(2013, 2, 15, 3, 0, 0, 1, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 3, 59, 59, 999999, tzinfo=tz.tzutc())

    def test_bounds_are_validated(self):

        with pytest.raises(ValueError):
            floor, ceil = self.atomic_clock.span("hour", bounds="][")

    def test_exact(self):

        result_floor, result_ceil = self.atomic_clock.span("hour", exact=True)

        expected_floor = datetime(2013, 2, 15, 3, 41, 22, 8923, tzinfo=tz.tzutc())
        expected_ceil = datetime(2013, 2, 15, 4, 41, 22, 8922, tzinfo=tz.tzutc())

        assert result_floor == expected_floor
        assert result_ceil == expected_ceil

    def test_exact_inclusive_inclusive(self):

        floor, ceil = self.atomic_clock.span("minute", bounds="[]", exact=True)

        assert floor == datetime(2013, 2, 15, 3, 41, 22, 8923, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 3, 42, 22, 8923, tzinfo=tz.tzutc())

    def test_exact_exclusive_inclusive(self):

        floor, ceil = self.atomic_clock.span("day", bounds="(]", exact=True)

        assert floor == datetime(2013, 2, 15, 3, 41, 22, 8924, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 16, 3, 41, 22, 8923, tzinfo=tz.tzutc())

    def test_exact_exclusive_exclusive(self):

        floor, ceil = self.atomic_clock.span("second", bounds="()", exact=True)

        assert floor == datetime(2013, 2, 15, 3, 41, 22, 8924, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 2, 15, 3, 41, 23, 8922, tzinfo=tz.tzutc())

    def test_all_parameters_specified(self):

        floor, ceil = self.atomic_clock.span("week", bounds="()", exact=True, count=2)

        assert floor == datetime(2013, 2, 15, 3, 41, 22, 8924, tzinfo=tz.tzutc())
        assert ceil == datetime(2013, 3, 1, 3, 41, 22, 8922, tzinfo=tz.tzutc())


class TestAtomicClockIsBetween:
    def test_start_before_end(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 8))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 5))
        assert not target.is_between(start, end)

    def test_exclusive_exclusive_bounds(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 5, 12, 30, 27))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 5, 12, 30, 10))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 5, 12, 30, 36))
        assert target.is_between(start, end, "()")

    def test_exclusive_exclusive_bounds_same_date(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        assert not target.is_between(start, end, "()")

    def test_inclusive_exclusive_bounds(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 6))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 4))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 6))
        assert not target.is_between(start, end, "[)")

    def test_exclusive_inclusive_bounds(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 5))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        assert target.is_between(start, end, "(]")

    def test_inclusive_inclusive_bounds_same_date(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        assert target.is_between(start, end, "[]")

    def test_inclusive_inclusive_bounds_target_before_start(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2020, 12, 24))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2020, 12, 25))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2020, 12, 26))
        assert not target.is_between(start, end, "[]")

    def test_type_error_exception(self):
        with pytest.raises(TypeError):
            target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
            start = datetime(2013, 5, 5)
            end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 8))
            target.is_between(start, end)

        with pytest.raises(TypeError):
            target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
            start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 5))
            end = datetime(2013, 5, 8)
            target.is_between(start, end)

        with pytest.raises(TypeError):
            target.is_between(None, None)

    def test_value_error_exception(self):
        target = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 7))
        start = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 5))
        end = atomic_clock.AtomicClock.fromdatetime(datetime(2013, 5, 8))
        with pytest.raises(ValueError):
            target.is_between(start, end, "][")
        with pytest.raises(ValueError):
            target.is_between(start, end, "")
        with pytest.raises(ValueError):
            target.is_between(start, end, "]")
        with pytest.raises(ValueError):
            target.is_between(start, end, "[")
        with pytest.raises(ValueError):
            target.is_between(start, end, "hello")
        with pytest.raises(ValueError):
            target.span("week", week_start=55)
