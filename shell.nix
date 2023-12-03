# vim: ts=3 sw=3 noet sts=0
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	nativeBuildInputs = with pkgs; [
		rustup gcc rustfmt clippy
		pkg-config
		clang
	];
	RUSTC_VERSION = "1.74.0";
	# https://github.com/rust-lang/rust-bindgen#environment-variables
	LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
	shellHook = ''
		export PATH="$PATH:''${CARGO_HOME:-~/.cargo}/bin"
		export PATH="$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/"
		'';
	# Add precompiled library to rustc search path
	RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
		# add libraries here (e.g. pkgs.libvmi)
	]);

	BINDGEN_EXTRA_CLANG_ARGS =
	# Includes with normal include path
	(builtins.map (a: ''-I"${a}/include"'') [
	# add dev libraries here (e.g. pkgs.libvmi.dev)
		pkgs.linux.dev
	]);

	#RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
