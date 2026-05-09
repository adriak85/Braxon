# NSQ Special Benchmark Checklist

A run is fair only if all are true:

- timed phase allows native compact output
- post-run decode is outside timed phase
- compact artifacts count as real output
- replay hash is preserved
- structural and relational recovery are scored
- future reuse is scored
- no same-shape cap is imposed
- no forced verbosity requirement is imposed
- no adapter burden is treated as base weakness
- no flattening against C has been introduced
