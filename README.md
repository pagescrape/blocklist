# blocklist [![CI](https://github.com/pagescrape/blocklist/actions/workflows/rust.yml/badge.svg)](https://github.com/pagescrape/blocklist/actions/workflows/rust.yml)

The project is based on blocklistproject. It provides perfect hash map/set structures for fast lookup of blocklisted items.

```rust,no_run
// Abuse
// https://blocklistproject.github.io/Lists/alt-version/abuse-nl.txt
//
// Drugs
// https://blocklistproject.github.io/Lists/dnsmasq-version/drugs-dnsmasq.txt
//
// Fraud
// https://blocklistproject.github.io/Lists/alt-version/fraud-nl.txt
//
// Gambinlig
// https://blocklistproject.github.io/Lists/alt-version/gambling-nl.txt
//
// Malware
// https://blocklistproject.github.io/Lists/alt-version/malware-nl.txt
//
// Phishing
// https://blocklistproject.github.io/Lists/alt-version/phishing-nl.txt
//
// Piracy
// https://blocklistproject.github.io/Lists/alt-version/piracy-nl.txt
//
// Porn
// https://blocklistproject.github.io/Lists/alt-version/porn-nl.txt
//
// Ransomware
// https://blocklistproject.github.io/Lists/alt-version/ransomware-nl.txt
//
// Redirect
// https://blocklistproject.github.io/Lists/alt-version/redirect-nl.txt
//
// Scam
// https://blocklistproject.github.io/Lists/alt-version/scam-nl.txt
//
// Torrent
// https://blocklistproject.github.io/Lists/alt-version/torrent-nl.txt
//
// Tracking
// https://blocklistproject.github.io/Lists/alt-version/tracking-nl.txt
//
// Ads
// https://blocklistproject.github.io/Lists/alt-version/ads-nl.txt

assert!(blocklist::is_advertisement("3003809.fls.doubleclick.net"));
assert!(!blocklist::is_advertisement("example.com"));

// To check if a domain is in any of the supported blocklists
assert!(blocklist::is_everything("123clickcash.com"));
assert!(blocklist::is_everything("3003809.fls.doubleclick.net"));
```
