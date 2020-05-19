{ stdenv, rustPlatform, fetchFromGitHub, llvmPackages, pkgconfig, imagemagick
, libiconv, makeWrapper
}:

rustPlatform.buildRustPackage rec {
  pname   = "dymoprint";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "puzzlewolf";
    repo  = "dymo_labelprinter";
    rev   = "2b26a3e02f331e3872f0ca99e571790606fa50b3";
    hash  = "sha256:1b07r3894vlq50pbvp4rjkkk13f7rizm7c5vrbpajnqfrzl0w5pc";
  };

  cargoSha256 = "sha256:1z2r6wr6vzayw60kx7f8mpxrid1s1ph6c989hvnz0baigcc8pynm";

  buildInputs = [ imagemagick ];

  postInstall = ''
    install -D -t $out/lib/udev/rules.d udev-rule/50-dymo.rules
  '';

  IM_CONVERT = "${imagemagick}/bin/convert";

  meta = with stdenv.lib; {
    description = "A utility to print labels on a Dymo label printer";
    homepage    = https://github.com/puzzlewolf/dymo_labelprinter;
    license     = with licenses; [ mit ];
    maintainers = with maintainers; [ puzzlewolf ];
    platforms   = platforms.all;
  };
}
