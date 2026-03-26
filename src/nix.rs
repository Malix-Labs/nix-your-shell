/// Arguments to a `nix` invocation.
#[derive(Debug)]
pub struct NixArgs {
    /// Arguments to the `nix` invocation, including the subcommand.
    pub args: Vec<String>,
    /// Subcommand to run, like `build` or `shell`.
    pub subcommand: Option<String>,
}

fn try_consume_option_values(
    args: &[String],
    ret: &mut Vec<String>,
    i: &mut usize,
    values_to_consume: usize,
) -> bool {
    if *i + values_to_consume >= args.len() {
        return false;
    }

    for offset in 1..=values_to_consume {
        ret.push(args[*i + offset].clone());
    }

    *i += values_to_consume;
    true
}

fn handle_end_of_options(args: &[String], ret: &mut Vec<String>, i: usize) -> bool {
    if args[i] == "--" {
        // End of options sentinel: copy the remaining positional arguments verbatim.
        ret.extend(args[i + 1..].iter().cloned());
        return true;
    }

    false
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OptionArity {
    Zero,
    One,
    Two,
}

const NIX_OPTIONS_TWO: &[&str] = &[
    "--option",
    "--redirect",
    "--override-flake",
    "--arg",
    "--argstr",
    "--override-input",
];

const NIX_OPTIONS_ONE: &[&str] = &[
    "--log-format",
    "--access-tokens",
    "--allowed-impure-host-deps",
    "--allowed-uris",
    "--allowed-users",
    "--bash-prompt",
    "--bash-prompt-prefix",
    "--bash-prompt-suffix",
    "--build-hook",
    "--build-poll-interval",
    "--build-users-group",
    "--builders",
    "--commit-lockfile-summary",
    "--connect-timeout",
    "--cores",
    "--diff-hook",
    "--download-attempts",
    "--download-speed",
    "--experimental-features",
    "--extra-access-tokens",
    "--extra-allowed-impure-host-deps",
    "--extra-allowed-uris",
    "--extra-allowed-users",
    "--extra-experimental-features",
    "--extra-extra-platforms",
    "--extra-hashed-mirrors",
    "--extra-nix-path",
    "--extra-platforms",
    "--extra-plugin-files",
    "--extra-sandbox-paths",
    "--extra-secret-key-files",
    "--extra-substituters",
    "--extra-system-features",
    "--extra-trusted-public-keys",
    "--extra-trusted-substituters",
    "--extra-trusted-users",
    "--flake-registry",
    "--gc-reserved-space",
    "--hashed-mirrors",
    "--http-connections",
    "--log-lines",
    "--max-build-log-size",
    "--max-free",
    "--max-jobs",
    "--max-silent-time",
    "--min-free",
    "--min-free-check-interval",
    "--nar-buffer-size",
    "--narinfo-cache-negative-ttl",
    "--narinfo-cache-positive-ttl",
    "--netrc-file",
    "--nix-path",
    "--plugin-files",
    "--post-build-hook",
    "--pre-build-hook",
    "--repeat",
    "--sandbox-paths",
    "--secret-key-files",
    "--stalled-download-timeout",
    "--store",
    "--substituters",
    "--system",
    "--system-features",
    "--tarball-ttl",
    "--timeout",
    "--trusted-public-keys",
    "--trusted-substituters",
    "--trusted-users",
    "--user-agent-suffix",
    // `nix develop` options
    "-k",
    "--keep",
    "--phase",
    "--profile",
    "--unset",
    "--eval-store",
    "-I",
    "--include",
    "--inputs-from",
    "--update-input",
    "--expr",
    "-f",
    "--file",
];

const NIX_OPTIONS_ZERO: &[&str] = &[
    "--offline",
    "--refresh",
    "--debug",
    "-L",
    "--print-build-logs",
    "--quiet",
    "-v",
    "--verbose",
    "--accept-flake-config",
    "--no-accept-flake-config",
    "--allow-dirty",
    "--no-allow-dirty",
    "--allow-import-from-derivation",
    "--no-allow-import-from-derivation",
    "--allow-symlinked-store",
    "--no-allow-symlinked-store",
    "--allow-unsafe-native-code-during-evaluation",
    "--no-allow-unsafe-native-code-during-evaluation",
    "--auto-optimise-store",
    "--no-auto-optimise-store",
    "--builders-use-substitutes",
    "--no-builders-use-substitutes",
    "--compress-build-log",
    "--no-compress-build-log",
    "--darwin-log-sandbox-violations",
    "--no-darwin-log-sandbox-violations",
    "--enforce-determinism",
    "--no-enforce-determinism",
    "--eval-cache",
    "--no-eval-cache",
    "--fallback",
    "--no-fallback",
    "--fsync-metadata",
    "--no-fsync-metadata",
    "--http2",
    "--no-http2",
    "--ignore-try",
    "--no-ignore-try",
    "--impersonate-linux-26",
    "--no-impersonate-linux-26",
    "--keep-build-log",
    "--no-keep-build-log",
    "--keep-derivations",
    "--no-keep-derivations",
    "--keep-env-derivations",
    "--no-keep-env-derivations",
    "--keep-failed",
    "--no-keep-failed",
    "--keep-going",
    "--no-keep-going",
    "--keep-outputs",
    "--no-keep-outputs",
    "--preallocate-contents",
    "--no-preallocate-contents",
    "--print-missing",
    "--no-print-missing",
    "--pure-eval",
    "--no-pure-eval",
    "--require-sigs",
    "--no-require-sigs",
    "--restrict-eval",
    "--no-restrict-eval",
    "--run-diff-hook",
    "--no-run-diff-hook",
    "--sandbox",
    "--no-sandbox",
    "--sandbox-fallback",
    "--no-sandbox-fallback",
    "--show-trace",
    "--no-show-trace",
    "--substitute",
    "--no-substitute",
    "--sync-before-registering",
    "--no-sync-before-registering",
    "--trace-function-calls",
    "--no-trace-function-calls",
    "--trace-verbose",
    "--no-trace-verbose",
    "--use-case-hack",
    "--no-use-case-hack",
    "--use-registries",
    "--no-use-registries",
    "--use-sqlite-wal",
    "--no-use-sqlite-wal",
    "--warn-dirty",
    "--no-warn-dirty",
    "--relaxed-sandbox",
    // `nix develop` options
    "--build",
    "--check",
    "--configure",
    "--debugger",
    "-i",
    "--ignore-environment",
    "--install",
    "--installcheck",
    "--unpack",
    "--impure",
    "--commit-lock-file",
    "--no-registries",
    "--no-update-lock-file",
    "--no-write-lock-file",
    "--recreate-lock-file",
    "--derivation",
];

const NIX_SHELL_OPTIONS_TWO: &[&str] = &[
    "--arg",
    "--argstr",
    // `nix-store`
    "--option",
    // From `nix-build` source...
    "--override-flake",
];

const NIX_SHELL_OPTIONS_ONE: &[&str] = &[
    "--attr",
    "-A",
    "--exclude",
    "--keep",
    "-i", // Interpreter, shebang only
    // `nix-store`
    "--add-root",
    // From `nix-build` source...
    "--cores",
    "--max-silent-time",
    "--timeout",
    "--store-uri",
    "-I",
    "--include",
    "--eval-store",
    "-o",
    "--out-link",
];

const NIX_SHELL_OPTIONS_ZERO: &[&str] = &[
    "--pure",
    "--impure",
    // `--packages` changes the meaning of positional arguments, so we effectively
    // ignore it.
    "-p",
    "--packages",
    // Also changes meaning of positional arguments.
    "-E",
    "--expr",
    // `nix-store`
    "--dry-run",
    "--ignore-unknown",
    "--check",
    // From `nix-build` source...
    "-Q",
    "--no-build-output",
    "-K",
    "--keep-failed",
    "-k",
    "--keep-going",
    "--fallback",
    "--readonly-mode",
    "--no-gc-warning",
    "--add-drv-link",
    "--indirect",
    "--no-out-link",
    "--no-link",
    "--drv-link",
    "--repair",
    "--run-env",
];

const NIX_SUBCOMMANDS: &[&str] = &[
    "build",
    "develop",
    "flake",
    "help",
    "profile",
    "repl",
    "run",
    "search",
    "shell",
    "bundle",
    "copy",
    "edit",
    "eval",
    "fmt",
    "log",
    "path-info",
    "registry",
    "why-depends",
    "daemon",
    "describe-stores",
    "hash",
    "key",
    "nar",
    "print-dev-env",
    "realisation",
    "show-config",
    "show-derivation",
    "store",
    "doctor",
    "upgrade-nix",
];

fn nix_option_arity(arg: &str) -> Option<OptionArity> {
    if NIX_OPTIONS_TWO.contains(&arg) {
        Some(OptionArity::Two)
    } else if NIX_OPTIONS_ONE.contains(&arg) {
        Some(OptionArity::One)
    } else if NIX_OPTIONS_ZERO.contains(&arg) {
        Some(OptionArity::Zero)
    } else {
        None
    }
}

fn nix_shell_option_arity(arg: &str) -> Option<OptionArity> {
    if NIX_SHELL_OPTIONS_TWO.contains(&arg) {
        Some(OptionArity::Two)
    } else if NIX_SHELL_OPTIONS_ONE.contains(&arg) {
        Some(OptionArity::One)
    } else if NIX_SHELL_OPTIONS_ZERO.contains(&arg) {
        Some(OptionArity::Zero)
    } else {
        None
    }
}

fn is_nix_command_override(arg: &str) -> bool {
    matches!(arg, "--help" | "--version" | "-c" | "--command")
}

fn is_nix_shell_command_override(arg: &str) -> bool {
    matches!(arg, "--command" | "--run" | "--help" | "--version")
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ParseOutcome {
    Continue,
    Break,
    ReturnOriginal,
}

fn consume_option_values(
    args: &[String],
    ret: &mut Vec<String>,
    i: &mut usize,
    arity: OptionArity,
) -> ParseOutcome {
    match arity {
        OptionArity::Two => {
            if !try_consume_option_values(args, ret, i, 2) {
                // Truncated option value(s); keep input unchanged and stop parsing.
                return ParseOutcome::Break;
            }
        }
        OptionArity::One => {
            if !try_consume_option_values(args, ret, i, 1) {
                // Truncated option value; keep input unchanged and stop parsing.
                return ParseOutcome::Break;
            }
        }
        OptionArity::Zero => {}
    }

    ParseOutcome::Continue
}

fn walk_args<F>(args: &[String], ret: &mut Vec<String>, mut on_arg: F) -> ParseOutcome
where
    F: FnMut(&str, &mut usize, &mut Vec<String>) -> ParseOutcome,
{
    let mut i = 0;
    while i < args.len() {
        ret.push(args[i].clone());

        if handle_end_of_options(args, ret, i) {
            return ParseOutcome::Break;
        }

        let arg = args[i].as_str();
        match on_arg(arg, &mut i, ret) {
            ParseOutcome::Continue => {}
            ParseOutcome::Break => return ParseOutcome::Break,
            ParseOutcome::ReturnOriginal => return ParseOutcome::ReturnOriginal,
        }

        i += 1;
    }

    ParseOutcome::Continue
}

/// Transform arguments to a `nix` invocation to run the specified `command` with the specified
/// `command_args`.
///
/// Only modifies `nix develop` and `nix shell` commands.
pub fn transform_nix(args: Vec<String>, command: &str, command_args: Vec<String>) -> NixArgs {
    let mut ret = Vec::with_capacity(args.len() + 2);

    let mut subcommand = None;

    let outcome = walk_args(&args, &mut ret, |arg, i, ret| {
        if is_nix_command_override(arg) {
            // We already have a command to run.
            return ParseOutcome::ReturnOriginal;
        }

        if let Some(arity) = nix_option_arity(arg) {
            return consume_option_values(&args, ret, i, arity);
        }

        if NIX_SUBCOMMANDS.contains(&arg) {
            // Top-level subcommand.

            // Replace `subcommand` unless it already has a value.
            subcommand.get_or_insert_with(|| args[*i].clone());
        } else {
            // Unknown argument, ignore.
        }

        ParseOutcome::Continue
    });

    if matches!(outcome, ParseOutcome::ReturnOriginal) {
        return NixArgs { args, subcommand };
    }

    // We want to add our `--command` flag right at the end, because `--command` makes *all the
    // rest of the positional arguments* get parsed as arguments to the command.
    //
    // Note that this behavior is unlike `nix-shell`, where the `--command` flag takes one argument
    // that may include spaces...
    match subcommand.as_deref() {
        Some("develop") | Some("shell") => {
            ret.push("--command".into());
            ret.push(command.into());
            ret.extend(command_args);
        }

        _ => {}
    }

    NixArgs {
        args: ret,
        subcommand,
    }
}

/// Transform arguments to a `nix-shell` invocation to run the specified `command` with the
/// specified `command_args`.
pub fn transform_nix_shell(
    args: Vec<String>,
    command: &str,
    command_args: &[String],
) -> Vec<String> {
    let mut ret = Vec::with_capacity(args.len() + 2);
    ret.push("--command".into());
    ret.push(shell_words::join(
        std::iter::once(command).chain(command_args.iter().map(|arg| arg.as_str())),
    ));

    let outcome = walk_args(&args, &mut ret, |arg, i, ret| {
        if is_nix_shell_command_override(arg) {
            // We already have a command to run; don't add our own `--command {command}`
            // arguments.
            return ParseOutcome::ReturnOriginal;
        }

        if let Some(arity) = nix_shell_option_arity(arg) {
            return consume_option_values(&args, ret, i, arity);
        }

        // Unknown argument, ignore.
        ParseOutcome::Continue
    });

    if matches!(outcome, ParseOutcome::ReturnOriginal) {
        return args;
    }

    ret
}
