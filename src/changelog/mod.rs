mod init;
mod merge;
mod parse;
mod write;

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub enum Section {
    /// A part of a changelog which couldn't be understood and is taken in verbatim. This is usually the pre-amble of the changelog
    /// or a custom footer.
    Verbatim {
        /// The section text, unchanged, up until the next `Release`.
        text: String,
        /// True if this is not created by a human
        generated: bool,
    },

    /// A segment describing a particular release
    Release {
        name: Version,
        date: Option<time::OffsetDateTime>,
        /// the amount of # in front of the heading denoting the release name
        heading_level: usize,
        /// How often we saw 'thanks clippy' as message
        thanks_clippy_count: usize,
        /// text of events of everything we couldn't parse
        unknown: String,
    },
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub enum Version {
    Unreleased,
    Semantic(semver::Version),
}

#[cfg(test)]
mod tests;
