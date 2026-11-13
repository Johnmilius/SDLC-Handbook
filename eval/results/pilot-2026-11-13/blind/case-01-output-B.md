# Tests for `max_retries` (src/config.rs)

These tests check the code against the task description, not against what the code currently does. Several of them fail on the current implementation. Those failures are real bugs, listed after the code.

Add these to the existing `mod tests` in `src/config.rs`.

```rust
    // ---- Config::set_max_retries: range ----

    #[test]
    fn default_is_three_when_never_set() {
        assert_eq!(Config::new().max_retries(), 3);
        assert_eq!(Config::default().max_retries(), 3);
    }

    #[test]
    fn set_accepts_lower_bound_zero() {
        let config = Config::new();
        assert_eq!(config.set_max_retries(0), Ok(()));
        assert_eq!(config.max_retries(), 0);
    }

    #[test]
    fn set_accepts_upper_bound_ten() {
        let config = Config::new();
        assert_eq!(config.set_max_retries(10), Ok(()));
        assert_eq!(config.max_retries(), 10);
    }

    #[test]
    fn set_accepts_every_value_in_range() {
        for v in 0..=10i64 {
            let config = Config::new();
            assert_eq!(config.set_max_retries(v), Ok(()), "value {v}");
            assert_eq!(config.max_retries() as i64, v);
        }
    }

    #[test]
    fn set_rejects_negative_values() {
        for v in [-1i64, -2, -255, -256, i64::MIN] {
            let config = Config::new();
            assert_eq!(config.set_max_retries(v), Err(ConfigError::OutOfRange), "value {v}");
        }
    }

    #[test]
    fn set_rejects_values_just_above_range() {
        for v in [11i64, 12, 100] {
            let config = Config::new();
            assert_eq!(config.set_max_retries(v), Err(ConfigError::OutOfRange), "value {v}");
        }
    }

    #[test]
    fn set_rejects_values_that_would_wrap_into_u8_range() {
        // 256 as u8 == 0, 259 as u8 == 3, 266 as u8 == 10, i64::MAX as u8 == 255.
        for v in [256i64, 259, 266, i64::MAX] {
            let config = Config::new();
            assert_eq!(config.set_max_retries(v), Err(ConfigError::OutOfRange), "value {v}");
        }
    }

    #[test]
    fn rejected_value_leaves_saved_value_unchanged() {
        let config = Config::new();
        config.set_max_retries(7).unwrap();
        for v in [-1i64, 11, 256] {
            assert!(config.set_max_retries(v).is_err());
            assert_eq!(config.max_retries(), 7, "after rejected {v}");
        }
    }

    #[test]
    fn last_valid_write_wins() {
        let config = Config::new();
        config.set_max_retries(2).unwrap();
        config.set_max_retries(9).unwrap();
        assert_eq!(config.max_retries(), 9);
    }

    // ---- put_max_retries: HTTP behavior ----

    const OUT_OF_RANGE_BODY: &str = r#"{"error": "max_retries_out_of_range"}"#;

    #[test]
    fn put_valid_value_returns_200_and_saves_it() {
        let config = Config::new();
        let resp = put_max_retries(&config, r#"{"value": 5}"#);
        assert_eq!(resp.status, 200);
        assert_eq!(config.max_retries(), 5);
    }

    #[test]
    fn put_accepts_both_bounds() {
        let config = Config::new();
        assert_eq!(put_max_retries(&config, r#"{"value": 0}"#).status, 200);
        assert_eq!(config.max_retries(), 0);
        assert_eq!(put_max_retries(&config, r#"{"value": 10}"#).status, 200);
        assert_eq!(config.max_retries(), 10);
    }

    #[test]
    fn put_out_of_range_returns_422_with_exact_body() {
        for body in [
            r#"{"value": 11}"#,
            r#"{"value": -1}"#,
            r#"{"value": 256}"#,
            r#"{"value": 99999999999}"#,
        ] {
            let config = Config::new();
            let resp = put_max_retries(&config, body);
            assert_eq!(resp.status, 422, "body {body}");
            assert_eq!(resp.body, OUT_OF_RANGE_BODY, "body {body}");
            assert_eq!(config.max_retries(), 3, "default must survive, body {body}");
        }
    }

    #[test]
    fn put_malformed_returns_422_with_exact_body() {
        // The spec says "out-of-range or malformed values" both get 422.
        for body in [
            "",
            "{}",
            "not json",
            r#"{"value": "5"}"#,
            r#"{"value": 5.5}"#,
            r#"{"value": null}"#,
            r#"{"value": true}"#,
            r#"{"value": }"#,
            r#"{"other": 5}"#,
            r#"{"value": 1e1}"#,
            r#"{"value": [5]}"#,
        ] {
            let config = Config::new();
            let resp = put_max_retries(&config, body);
            assert_eq!(resp.status, 422, "body {body:?}");
            assert_eq!(resp.body, OUT_OF_RANGE_BODY, "body {body:?}");
            assert_eq!(config.max_retries(), 3, "default must survive, body {body:?}");
        }
    }

    #[test]
    fn put_rejects_values_that_only_look_valid_after_parsing_loosely() {
        // A prefix parse would read these as 5; they are not valid integers.
        for body in [r#"{"value": 5abc}"#, r#"{"value": 5 5}"#, r#"{"value": "5"}"#] {
            let config = Config::new();
            assert_eq!(put_max_retries(&config, body).status, 422, "body {body}");
            assert_eq!(config.max_retries(), 3, "body {body}");
        }
    }

    // ---- RetryWorker: must use the saved value ----
    //
    // The worker currently reads the constant `MAX_RETRIES` and has no access to
    // `Config`, so these cannot pass until it is wired to the config. They assume
    // `RetryWorker::new(&Config)` (or similar) that reads `config.max_retries()`
    // when a job starts. Adjust the constructor to whatever shape you choose.

    #[test]
    fn worker_makes_one_attempt_plus_max_retries_when_job_always_fails() {
        let config = Config::new();
        for n in [0u8, 1, 3, 10] {
            config.set_max_retries(n.into()).unwrap();
            let worker = RetryWorker::new(&config);
            assert_eq!(worker.run(|| false), u32::from(n) + 1, "max_retries {n}");
        }
    }

    #[test]
    fn worker_with_zero_retries_runs_job_once() {
        let config = Config::new();
        config.set_max_retries(0).unwrap();
        let worker = RetryWorker::new(&config);
        let mut calls = 0;
        worker.run(|| { calls += 1; false });
        assert_eq!(calls, 1);
    }

    #[test]
    fn worker_uses_default_when_never_set() {
        let config = Config::new();
        let worker = RetryWorker::new(&config);
        assert_eq!(worker.run(|| false), 4);
    }

    #[test]
    fn worker_stops_as_soon_as_job_succeeds() {
        let config = Config::new();
        config.set_max_retries(10).unwrap();
        let worker = RetryWorker::new(&config);
        let mut calls = 0;
        let attempts = worker.run(|| { calls += 1; calls == 2 });
        assert_eq!(attempts, 2);
        assert_eq!(calls, 2);
    }

    #[test]
    fn job_started_after_change_uses_new_value() {
        let config = Config::new();
        let worker = RetryWorker::new(&config);
        assert_eq!(worker.run(|| false), 4); // default 3 retries
        assert_eq!(put_max_retries(&config, r#"{"value": 6}"#).status, 200);
        assert_eq!(worker.run(|| false), 7); // next job sees 6 retries
    }

    #[test]
    fn rejected_put_does_not_change_worker_behavior() {
        let config = Config::new();
        let worker = RetryWorker::new(&config);
        assert_eq!(put_max_retries(&config, r#"{"value": 11}"#).status, 422);
        assert_eq!(worker.run(|| false), 4);
    }

    #[test]
    fn change_during_a_running_job_does_not_affect_that_job() {
        // "Jobs it starts after the change": a job already running keeps the limit
        // it started with. Requires the worker to read the value once per run.
        let config = Config::new();
        config.set_max_retries(2).unwrap();
        let worker = RetryWorker::new(&config);
        let mut calls = 0;
        let attempts = worker.run(|| {
            calls += 1;
            if calls == 1 {
                config.set_max_retries(10).unwrap();
            }
            false
        });
        assert_eq!(attempts, 3);
    }
```

## What these tests find in the current code

`cargo test` passing earlier only means the three existing tests agree with the code. Run against the description, these fail:

1. **Upper bound is off by one.** `value < 10` rejects 10. The spec says 0 to 10 inclusive. Fix: `(0..=10).contains(&value)`. Fails `set_accepts_upper_bound_ten` and `put_accepts_both_bounds`.
2. **No lower bound, and negatives wrap.** `-1` passes `< 10` and `-1 as u8` stores 255. Same fix as above. Fails `set_rejects_negative_values` and the `-1` rows in the PUT tests.
3. **The worker ignores the setting.** `RetryWorker::run` reads the constant `MAX_RETRIES` (always 3) and has no `Config`. The requirement that jobs started after a change use the saved value is not implemented. Fails every test in the worker section (they will not compile until the worker takes a config, which is the signal). The existing `worker_stops_after_retry_limit` only confirms the hard-coded 3.
4. **Malformed input returns 400, not 422.** `put_max_retries` answers `{"error": "bad_request"}` with 400, but the spec says malformed values get 422 with `max_retries_out_of_range`. Fails `put_malformed_returns_422_with_exact_body`.
5. **Parsing is too loose.** `parse_value` splits on text, so `{"value": 5abc}` fails correctly, but things like a second key or a nested object can be mis-read (for example `{"other": {"value": 5}}` reads 5). A real JSON parser (`serde_json`) with a strict integer check is safer. The 5.5, `"5"`, `null` and `true` cases in the malformed test guard this.
6. **Redundant constants.** `DEFAULT_MAX_RETRIES` and `MAX_RETRIES` are both 3 and mean different things. Once the worker reads the config, `MAX_RETRIES` should go.

Note on the success body: the spec does not say what a 200 returns. The tests only check status 200 for success; add a body assertion once that is decided.

One design decision to make before the worker tests compile: the worker needs a way to see the config (a `&Config` or `Arc<Config>`). The tests use `RetryWorker::new(&config)`, which is a placeholder for that choice.
