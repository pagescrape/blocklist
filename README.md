# blocklist

The project is based on blocklistproject. It provides perfect hash map/set structures for fast lookup of blocklisted items.

```rust,no_run
assert!(blocklist::is_advertisement("3003809.fls.doubleclick.net"));
assert!(!blocklist::is_advertisement("example.com"));
```
