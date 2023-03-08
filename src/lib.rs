mod ip_filter;
use std::{net::Ipv4Addr, str::FromStr};
mod ipv4_with_properties;
pub use ipv4_with_properties::Ipv4WithProperties;

use colored::{ColoredString, Colorize};
pub use ip_filter::IpFilter;
use lazy_regex::regex;

pub fn format_ipv4(
    excludes: &[IpFilter],
    includes: &[IpFilter],
    ignore_ips: &[Ipv4Addr],
    line: &str,
) -> Option<String> {
    let ipv4_regex = regex!(
        r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"
    );

    let mut display_this_line = false;

    if !ipv4_regex.is_match(line) {
        return None;
    }

    if excludes.contains(&IpFilter::IPv4) {
        return None;
    }

    let mut result = String::new();
    let mut last_end = 0;

    for m in ipv4_regex.find_iter(line) {
        // Sometimes there are values like OIDs, which look like IPv4 addresses.
        // We can detect those by doing a look-ahead resp. look-behind. But because
        // are not supported by the regex create, we do this by hand
        if m.start() > 0 && line.as_bytes()[m.start() - 1] == b'.'
            || m.end() < line.len() - 1 && line.as_bytes()[m.end() + 1] == b'.'
        {
            result.push_str(&line[last_end..m.start()]);
            result.push_str(m.as_str());
            last_end = m.end();
            continue;
        }

        let ip_addr = match Ipv4Addr::from_str(m.as_str()) {
            Ok(addr) => Ipv4WithProperties::from(addr),
            Err(_) => {
                log::warn!("invalid IP address: {}, don't highlighting it", m.as_str());
                continue;
            }
        };

        if !ignore_ips.contains(ip_addr.as_ref()) {
            if excludes.contains(&IpFilter::Loopback) && ip_addr.is_loopback() {
                return None;
            }

            if excludes.contains(&IpFilter::Private) && ip_addr.is_private() {
                return None;
            }

            if excludes.contains(&IpFilter::Public) && ip_addr.is_global() {
                return None;
            }

            display_this_line |= includes.contains(&IpFilter::Loopback) && ip_addr.is_loopback();
            display_this_line |= includes.contains(&IpFilter::Private) && ip_addr.is_private();
            display_this_line |= includes.contains(&IpFilter::Public) && ip_addr.is_global();
        }

        // add the non-matching string between the last match and the current match to the result
        result.push_str(&line[last_end..m.start()]);
        last_end = m.end();

        if ignore_ips.contains(ip_addr.as_ref()) {
            result.push_str(m.as_str());
        } else {
            let highlighted_address: ColoredString = if ip_addr.is_global() {
                m.as_str().red().on_bright_yellow()
            } else {
                m.as_str().bright_purple()
            };
            #[allow(clippy::unnecessary_to_owned)]
            result.push_str(&(highlighted_address.to_string()));
        }
    }

    if last_end < line.as_bytes().len() {
        result.push_str(&line[last_end..]);
    }

    if display_this_line {
        Some(result)
    } else {
        None
    }
}
