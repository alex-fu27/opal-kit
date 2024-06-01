{
	inputs = {
		nixpkgs.url = "nixpkgs/nixos-24.05";
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

					LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

					# https://github.com/rust-lang/rust-bindgen#environment-variables
					BINDGEN_EXTRA_CLANG_ARGS =
					(builtins.map (a: ''-I"${a}/include"'') [
						# TODO is this still using glib headers?
						# use the default kernel package of the current nixos here to (at a minimum) support most nixos installations
						pkgs.linuxPackages.kernel.dev
					]);
				};
			});
}
