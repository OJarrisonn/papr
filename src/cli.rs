use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about)]
/// A simple CLI syntax highlighting tool for email files
///
/// papr works by taking an email file (.mbx) as input (either piped in or specified as an argument)
/// outputting it by appling stylization to it's header.
///
/// papr has some special highlighting for email files that are patches (diffs).
///
pub struct Args {
    /// The files to apply syntax highlighting to
    pub files: Vec<String>,
    /// Reduce messages to show only it's frontmatter
    #[clap(short, long)]
    pub frontmatter: bool,
}
