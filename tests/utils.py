def assert_datetime_equality(dt1, dt2, within=10):
    assert abs((dt1 - dt2).total_seconds()) < within
