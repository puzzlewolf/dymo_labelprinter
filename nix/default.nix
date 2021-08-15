{ stdenv, rustPlatform, fetchFromGitHub, llvmPackages, pkgconfig, imagemagick
, libiconv, makeWrapper }:

rustPlatform.buildRustPackage rec {
  pname = "dymoprint";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "puzzlewolf";
    repo = "dymo_labelprinter";
    rev = "ead1ba97df6bd2c4aaff6c00998f5318719e0255";
    hash = "sha256:0j3ziixpv33552cwwsjbjmbyvy39ishd7qj3wbkyvv56yrmxbdcj";
  };

  cargoSha256 = "sha256:0j98fy18ksa5crqdzv06v2gqmxzhhi103ib88xk3n1cq2cz87qsy";

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
