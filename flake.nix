{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        bin =  craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./.;
        # Add extra inputs here or any other derivation settings
        # doCheck = true;
        buildInputs = with pkgs;[sqlite];
        # nativeBuildInputs = [];
      };
      in
    {
      packages.default =bin;
      packages.dockerImage = pkgs.dockerTools.buildImage {
            name = "short";
            tag = "latest";
            copyToRoot =  pkgs.buildEnv {
    name = "image-root";
    pathsToLink = ["/"];
    paths =  with pkgs.dockerTools;[ ./container] ++ [
                  bin 
                  binSh
                  usrBinEnv
                  pkgs.coreutils
                  caCertificates
                  fakeNss
                  pkgs.wget
            ];
  };            config = {
              Cmd = [ "/bin/short" ];
            };
        };
    });
}
