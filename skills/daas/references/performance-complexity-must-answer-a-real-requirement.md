# Performance complexity must answer a real requirement

Start with the simplest semantically correct design. Measure before accepting
additional code or representation complexity for speed.

Optimization carries ongoing maintenance cost, not just an initial
implementation cost. Doing less work and tracking less unnecessary data often
help both performance and simplicity.

Arenas, zero initialization, or a larger optimized implementation can be
justified by a concrete benchmarked need. Search for alternatives before taking
on a dramatic increase in owned code. If performance is essential to the
product's purpose, necessary complexity may be part of the minimum solution.

Do not turn an occasional dual-implementation differential oracle into a
universal requirement. Dan often uses one for a port and then retires it.
