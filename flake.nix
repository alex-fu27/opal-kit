{
	inputs = {
		nixpkgs.url = "nixpkgs/nixpkgs-unstable";
		utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, utils }:
		utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };
			in
			{
				devShell = with pkgs; mkShell {
					nativeBuildInputs = with pkgs; [
						cargo rustc rustfmt rustPackages.clippy clang
					];

					RUSTC_VERSION = "1.74.0";

					# https://github.com/rust-lang/rust-bindgen#environment-variables
					BINDGEN_EXTRA_CLANG_ARGS =
					(builtins.map (a: ''-I"${a}/include"'') [
					# TODO make bindgen use linux headers instead of glib headers
						pkgs.linux.dev
					]);
				};
			});
}
