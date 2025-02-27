/// this module generate the clap App, which defines
/// launch arguments

/// declare the possible CLI arguments
pub fn clap_app() -> clap::App<'static, 'static> {
    let app = clap::App::new("broot")
        .version(env!("CARGO_PKG_VERSION"))
        .author("dystroy <denys.seguret@gmail.com>")
        .about(
            "A tree explorer and a customizable launcher\n\
            Complete documentation lives at https://dystroy.org/broot"
        )
        .setting(clap::AppSettings::ColoredHelp)
        .arg(clap::Arg::with_name("ROOT").help("sets the root directory"))
        // tree flags
        .arg(
            clap::Arg::with_name("dates")
                .short("d")
                .long("dates")
                .help("Show the last modified date of files and directories"),
        )
        .arg(
            clap::Arg::with_name("no-dates")
                .short("D")
                .long("no-dates")
                .help("Don't show last modified date"),
        )
        .arg(
            clap::Arg::with_name("only-folders")
                .short("f")
                .long("only-folders")
                .help("Only show folders"),
        )
        .arg(
            clap::Arg::with_name("no-only-folders")
                .short("F")
                .long("no-only-folders")
                .help("Show folders and files alike"),
        )
        .arg(
            clap::Arg::with_name("show-root-fs")
                .long("show-root-fs")
                .help("Show filesystem info on top"),
        )
        .arg(
            clap::Arg::with_name("show-git-info")
                .short("g")
                .long("show-git-info")
                .help("Show git statuses on files and stats on repo"),
        )
        .arg(
            clap::Arg::with_name("no-show-git-info")
                .short("G")
                .long("no-show-git-info")
                .help("Don't show git statuses on files"),
        )
        .arg(
            clap::Arg::with_name("git-status")
                .long("git-status")
                .help("Only show files having an interesting git status, including hidden ones"),
        )
        .arg(
            clap::Arg::with_name("hidden")
                .short("h")
                .long("hidden")
                .help("Show hidden files"),
        )
        .arg(
            clap::Arg::with_name("no-hidden")
                .short("H")
                .long("no-hidden")
                .help("Don't show hidden files"),
        )
        .arg(
            clap::Arg::with_name("show-gitignored")
                .short("i")
                .long("show-gitignored")
                .help("Show files which should be ignored according to git"),
        )
        .arg(
            clap::Arg::with_name("no-show-gitignored")
                .short("I")
                .long("no-show-gitignored")
                .help("Don't show gitignored files"),
        )
        .arg(
            clap::Arg::with_name("permissions")
                .short("p")
                .long("permissions")
                .help("Show permissions, with owner and group"),
        )
        .arg(
            clap::Arg::with_name("no-permissions")
                .short("P")
                .long("no-permissions")
                .help("Don't show permissions"),
        )
        .arg(
            clap::Arg::with_name("sizes")
                .short("s")
                .long("sizes")
                .help("Show the size of files and directories"),
        )
        .arg(
            clap::Arg::with_name("no-sizes")
                .short("S")
                .long("no-sizes")
                .help("Don't show sizes"),
        )
        .arg(
            clap::Arg::with_name("sort-by-count")
                .long("sort-by-count")
                .help("Sort by count (only show one level of the tree)"),
        )
        .arg(
            clap::Arg::with_name("sort-by-date")
                .long("sort-by-date")
                .help("Sort by date (only show one level of the tree)"),
        )
        .arg(
            clap::Arg::with_name("sort-by-size")
                .long("sort-by-size")
                .help("Sort by size (only show one level of the tree)"),
        )
        .arg(
            clap::Arg::with_name("whale-spotting")
                .short("w")
                .long("whale-spotting")
                .help("Sort by size, show ignored and hidden files"),
        )
        .arg(
            clap::Arg::with_name("no-sort")
                .long("no-sort")
                .help("Don't sort"),
        )
        .arg(
            clap::Arg::with_name("trim-root")
                .short("t")
                .long("trim-root")
                .help("Trim the root too and don't show a scrollbar"),
        )
        .arg(
            clap::Arg::with_name("no-trim-root")
                .short("T")
                .long("no-trim-root")
                .help("Don't trim the root level, show a scrollbar"),
        )
        // other options
        .arg(
            clap::Arg::with_name("cmd-export-path")
                .long("outcmd")
                .takes_value(true)
                .help("Where to write the produced cmd (if any)"),
        )
        .arg(
            clap::Arg::with_name("commands")
                .short("c")
                .long("cmd")
                .takes_value(true)
                .help("Semicolon separated commands to execute"),
        )
        .arg(
            clap::Arg::with_name("color")
                .long("color")
                .takes_value(true)
                .possible_values(&["yes", "no", "auto"])
                .default_value("auto")
                .help("Whether to have styles and colors (auto is default and usually OK)"),
        )
        .arg(
            clap::Arg::with_name("conf")
                .long("conf")
                .takes_value(true)
                .help("Semicolon separated paths to specific config files"),
        )
        .arg(
            clap::Arg::with_name("height")
                .long("height")
                .help("Height (if you don't want to fill the screen or for file export)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("file-export-path")
                .short("o")
                .long("out")
                .takes_value(true)
                .help("Where to write the produced path (if any)"),
        )
        .arg(
            clap::Arg::with_name("install")
                .long("install")
                .help("Install or reinstall the br shell function"),
        )
        .arg(
            clap::Arg::with_name("no-style")
                .hidden(true) // we're deprecating this in favor of --color
                .long("no-style")
                .help("Whether to remove all style and colors from exported tree"),
        )
        .arg(
            clap::Arg::with_name("set-install-state")
                .long("set-install-state")
                .takes_value(true)
                .value_name("state")
                .possible_values(&["undefined", "refused", "installed"])
                .help("Set the installation state (for use in install script)"),
        )
        .arg(
            clap::Arg::with_name("print-shell-function")
                .long("print-shell-function")
                .takes_value(true)
                .value_name("shell")
                .help("Print to stdout the br function for a given shell"),
        )
        .setting(clap::AppSettings::DeriveDisplayOrder);
    #[cfg(feature="client-server")]
    let app = app
        .arg(
            clap::Arg::with_name("listen")
            .long("listen")
            .takes_value(true)
            .help("Listen for commands")
        )
        .arg(
            clap::Arg::with_name("get-root")
            .long("get-root")
            .help("Ask for the current root of the remote broot")
        )
        .arg(
            clap::Arg::with_name("send")
            .long("send")
            .takes_value(true)
            .help("send command and quits")
        );
    app
}
