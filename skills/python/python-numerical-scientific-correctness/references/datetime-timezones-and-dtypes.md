# Timezones, Datetimes, And Dtypes

Timezone-aware timestamps and dtype/missing-data coercion in pandas/numpy.

- A timezone-aware timestamp carries an explicit offset; a naive timestamp does not and is interpreted against the local environment, so pandas raises or misbehaves when naive and aware timestamps are combined and DST transitions shift naive values.
- The pandas time-zone handling guidance is to localize/convert to an explicit zone (UTC internally) and convert only for display; `tz_localize` attaches a zone and `tz_convert` changes it.
- In pandas/numpy, integer arrays have no missing-value sentinel, so introducing `NaN` upcasts an integer column to float (or object); a nullable integer dtype preserves integer semantics while allowing missing values.

## Sources

- https://pandas.pydata.org/docs/user_guide/timeseries.html#time-zone-handling
- https://numpy.org/doc/stable/user/basics.types.html
