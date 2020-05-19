{ stdenv, rustPlatform, fetchFromGitHub, llvmPackages, pkgconfig, imagemagick
, libiconv, makeWrapper
}:

rustPlatform.buildRustPackage rec {
  pname   = "dymoprint";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "puzzlewolf";
    repo  = "dymo_labelprinter";
    rev   = "fc8cd95bcbcda6e549c7bb3f4f546f6d5e679295";
    hash  = "sha256:0fxd5sa4csia7k291kicwf5lzv2w94i6k2xsj7pxmqhr2k1mq7ff";
  };

  cargoSha256 = "sha256:1z2r6wr6vzayw60kx7f8mpxrid1s1ph6c989hvnz0baigcc8pynm";

  buildInputs = [ imagemagick ];

  postInstall = ''
    install -D -t $out/lib/udev/rules.d udev_rule/50-dymo.rules
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
