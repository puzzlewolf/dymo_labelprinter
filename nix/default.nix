{ stdenv, rustPlatform, fetchFromGitHub, llvmPackages, pkgconfig, imagemagick
, libiconv, makeWrapper }:

rustPlatform.buildRustPackage rec {
  pname = "dymoprint";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "puzzlewolf";
    repo = "dymo_labelprinter";
    rev = "f951074b4967abca88e5d03f9df506e76a8168df";
    hash = "sha256:1d2bl6klfk2krh4l38w2j0pxvi81gd6rnx1cmv9phv3alvrgld66";
  };

  cargoSha256 = "sha256:1z2r6wr6vzayw60kx7f8mpxrid1s1ph6c989hvnz0baigcc8pynm";

  buildInputs = [ imagemagick ];

  postInstall = ''
    install -D -t $out/lib/udev/rules.d udev_rule/50-dymo.rules
  '';

  IM_CONVERT = "${imagemagick}/bin/convert";

  meta = with stdenv.lib; {
    description = "A utility to print labels on a Dymo label printer";
    homepage = "https://github.com/puzzlewolf/dymo_labelprinter";
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ puzzlewolf ];
    platforms = platforms.all;
  };
}
