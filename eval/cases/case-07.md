# Case 07: password-reset tokens that expire

## Plain request

> Write tests for this.

## Material (given to both conditions)

The task:

> Add password-reset tokens that expire after 30 minutes.

A draft implementation exists. `cargo test` passes (there are no tests for this module yet).

```rust
// src/reset.rs

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

const TOKEN_TTL: Duration = Duration::from_secs(30 * 60);

pub struct ResetTokens {
    // token -> (user id, time issued)
    issued: HashMap<String, (u64, SystemTime)>,
}

impl ResetTokens {
    pub fn new() -> Self {
        ResetTokens { issued: HashMap::new() }
    }

    /// Creates a reset token for the user and returns it. The caller emails it.
    pub fn issue(&mut self, user_id: u64) -> String {
        let token = format!("{:08x}", fastrand::u32(..));
        self.issued.insert(token.clone(), (user_id, SystemTime::now()));
        token
    }

    /// Returns the user id if the token is valid.
    pub fn redeem(&self, candidate: &str) -> Option<u64> {
        for (token, (user_id, issued_at)) in &self.issued {
            if token == candidate {
                let age = SystemTime::now().duration_since(*issued_at).ok()?;
                if age < TOKEN_TTL {
                    return Some(*user_id);
                }
            }
        }
        None
    }
}
```

`redeem` is called by `POST /password/reset`, which then sets the new password.
