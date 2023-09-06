/// Hardware Detection Logic
/// Borrowed (stolen) from llm-cli
/// https://github.com/rustformers/llm/blob/main/binaries/llm-cli/src/cli_args.rs#L353

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub fn autodetect_num_threads() -> usize {
	std::process::Command::new("sysctl")
		.arg("-n")
		.arg("hw.perflevel0.physicalcpu")
		.output()
		.ok()
		.and_then(|output| String::from_utf8(output.stdout).ok()?.trim().parse().ok())
		.unwrap_or(num_cpus::get_physical())
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
pub fn autodetect_num_threads() -> usize {
	num_cpus::get_physical()
}
