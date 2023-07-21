//! Blocklist is based on blocklistproject provides it perfect hash map/set
//! structures for fast lookup of blocklisted items.
#![deny(
    warnings,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    deprecated,
    unconditional_recursion,
    unknown_lints,
    unreachable_code,
    unused_mut
)]

include!(concat!(env!("OUT_DIR"), "/blocklist-ads.rs"));

/// Check if domain is an advertisement
/// Note that parsing domain is not a part of this crate
/// you should use some other crate for that, e.g.: URL.
pub fn is_advertisement(domain: &str) -> bool {
    BLOCKLIST_ADS_LINKS.contains(&domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_advertisement() {
        assert!(is_advertisement("000lp59.wcomhost.com"));
        assert!(is_advertisement("3003809.fls.doubleclick.net"));
        assert!(is_advertisement("mini6g.com"));
        assert!(!is_advertisement("example.com"));
    }
}
