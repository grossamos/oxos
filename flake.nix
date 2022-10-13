{
  description = "A rust based operating system for the raspberry pi";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/nixos-22.05;

  outputs = { self, nixpkgs }: 
  let
    supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; }); 
  in
  {

    defaultPackage.x86_64-linux = 
      with import nixpkgs { system = "x86_64-linux"; };
      stdenv.mkDerivation {
        name = "hello";
        src = self;
      };
      devShell = forAllSystems(system: 
        nixpkgsFor.${system}.mkShell {
          buildInputs = [ 
            nixpkgsFor.${system}.qemu
            nixpkgsFor.${system}.xz
          ];
        }
      ); 

    dispositionPackage =
      with import nixpkgs { system = "x86_64-linux"; };
      stdenv.mkDerivation {
        name = "disposition";
        src = ./dispositionspapier;
        buildInputs = [
          nixpkgsFor.${system}.coreutils
          nixpkgsFor.${system}.tex
        ];
      };
  };
}
