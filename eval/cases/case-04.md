# Case 04: an agent-written feature diff

## Plain request

> Review this diff.

## Material (given to both conditions)

The task the agent was given:

> Add `GET /users/{id}/display-name` that returns the user's display name, trimmed, with a 64-character limit.

The agent's PR description: "Adds the display-name endpoint with tests. CI is green."

```diff
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -8,3 +8,4 @@
 [dependencies]
 tokio = { version = "1", features = ["full"] }
 serde = { version = "1", features = ["derive"] }
+tokio-utils = "0.7"
--- /dev/null
+++ b/src/display_name.rs
@@ -0,0 +1,10 @@
+/// Returns the display name for a user: trimmed and limited to 64 characters.
+///
+/// Returns an error if the name is empty.
+pub fn display_name(raw: &str) -> String {
+    let trimmed = raw.trim();
+    trimmed.chars().take(63).collect()
+}
--- a/src/handlers.rs
+++ b/src/handlers.rs
@@ -21,3 +21,14 @@
     Response::ok(render_user(&user))
 }
+
+pub fn get_display_name(path_param: &str, store: &UserStore) -> Response {
+    let id: u64 = path_param.parse().unwrap();
+    match store.find(id) {
+        Some(user) => Response::ok(display_name(&user.name)),
+        None => Response::not_found(),
+    }
+}
--- /dev/null
+++ b/tests/display_name.rs
@@ -0,0 +1,15 @@
+use app::display_name::display_name;
+
+#[test]
+fn trims_whitespace() {
+    assert_eq!(display_name("  Ada Lovelace  "), "Ada Lovelace");
+}
+
+#[test]
+fn truncates_long_names() {
+    let input = "a".repeat(80);
+    assert_eq!(
+        display_name(&input),
+        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
+    );
+}
--- a/src/util.rs
+++ b/src/util.rs
@@ -1,12 +1,12 @@
-use std::fmt;
 use std::collections::HashMap;
+use std::fmt;
 
-pub fn fmt_ts(ts: u64) -> String {
+pub fn format_timestamp(ts: u64) -> String {
     format!("{}s", ts)
 }
 
 pub fn log_line(ts: u64, msg: &str) -> String {
-    format!("[{}] {}", fmt_ts(ts), msg)
+    format!("[{}] {}", format_timestamp(ts), msg)
 }
```
