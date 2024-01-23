{
  description = "A very basic flake";

  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default =
        let
          python = pkgs.python3;
          pythonWithPackages = python.withPackages
            (p: with p; [
              z3
              requests
            ]);
        in
        pkgs.mkShell {
          buildInputs = with pkgs; [
            pythonWithPackages
            nodejs
          ];
        };
    };
}
