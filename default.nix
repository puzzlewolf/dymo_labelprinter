{ lib, rustPlatform, imagemagick }:

rustPlatform.buildRustPackage rec {
  pname = "dymoprint";
  version = "0.1.0";

  src = ./.;
  cargoLock = { lockFile = ./Cargo.lock; };

  buildInputs = [ imagemagick ];

  postInstall = ''
    install -D -t $out/lib/udev/rules.d udev_rule/50-dymo.rules
  '';

  IM_CONVERT = "${imagemagick}/bin/convert";

  meta = with lib; {
    description = "A utility to print labels on a Dymo label printer";
    homepage = "https://github.com/puzzlewolf/dymo_labelprinter";
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ puzzlewolf ];
    platforms = platforms.all;
  };
}
