# bench/

Performance and correctness gates for cgx (`gen_corpus.py`, `perf_gate.sh`, etc.) plus their
synthetic corpora and results.

## Validation runs against large/external corpora

For a one-off validation pass against a corpus too large or too specific to vendor here (e.g. a
third-party codebase used to reproduce a memory/performance issue), write run output to
`bench/validation-runs/<date>-<target>/` — this directory is gitignored. Keep raw logs, RSS/timing
traces, and graph stats there; they're reproducible from the corpus + the commit under test, so
there's no need to commit them.

If a result is worth keeping long-term (it justified a fix, or it's a regression baseline),
add a short, hand-curated summary to this README or to the relevant PR/commit message instead of
committing the raw run output. `perf_gate_results.txt` in this directory is the exception — it's
the committed, append-only log for the small synthetic-corpus perf gate (`perf_gate.sh`), which is
cheap enough to run and small enough to commit directly; don't extend that pattern to larger
one-off validation runs.
