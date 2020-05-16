{ stdenv, rustPlatform, fetchFromGitHub, llvmPackages, pkgconfig, imagemagick
, libiconv, makeWrapper
}:

rustPlatform.buildRustPackage rec {
  pname   = "dymoprint";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "puzzlewolf";
    repo  = "dymo_labelprinter";
    rev   = "7eb3cd663d5732622f908fa8017fcc6788d70102";
    hash  = "sha256:0y6blw5qjds4hikbdlh82xlwb42dbf8y59qank8hbkbl1rr7wjil";
  };

  cargoSha256 = "sha256:1vdyf2xjh59kpr5qak8qhmvflhl9zf3xmqb9vyznl45az0ymnrnl";

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
