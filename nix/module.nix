{ lib, pkgs, config, ... }:
with lib;
let
  cfg = config.services.dymoprint;
in {
  options.services.dymoprint = {
    enable = mkEnableOption "dymoprint service";
    port = mkOption {
      type = types.int;
      default = 8080;
      description = ''
        Port on which the Dymo printer webservice should listen.
      '';
    };
    openFirewall = mkOption {
      type = types.bool;
      default = false;
      description = ''
        Whether to automatically open the specified ports in the firewall.
      '';
    };

  };

  config = lib.mkIf cfg.enable {
    networking.firewall.allowedTCPPorts = if cfg.openFirewall then [ cfg.port ] else [];

    systemd.services.dymoprint = {
      wantedBy = [ "multi-user.target" ];
      serviceConfig.ExecStart = "${pkgs.dymoprint}/bin/dymo_print_server -a 0.0.0.0 -p ${toString cfg.port}";
      serviceConfig.User = "nobody";
    };

    services.udev.packages = [ dymoprint ];
  };
}
