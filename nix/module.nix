{ tableaux-playground }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.tableaux-playground;

  # Taken from immich
  commonServiceConfig = {
    Type = "simple";

    # Hardening
    CapabilityBoundingSet = "";
    NoNewPrivileges = true;
    PrivateUsers = true;
    PrivateTmp = true;
    PrivateDevices = true;
    PrivateMounts = true;
    ProtectClock = true;
    ProtectControlGroups = true;
    ProtectHome = true;
    ProtectHostname = true;
    ProtectKernelLogs = true;
    ProtectKernelModules = true;
    ProtectKernelTunables = true;
    RestrictAddressFamilies = [
      "AF_INET"
      "AF_INET6"
      "AF_UNIX"
    ];
    RestrictNamespaces = true;
    RestrictRealtime = true;
    RestrictSUIDSGID = true;
  };
in
{
  options.services.tableaux-playground = {
    enable = lib.mkEnableOption "tableaux-playground";
    port = lib.mkOption {
      type = lib.types.int;
      default = 9891;
      description = "Port to listen on";
    };

    host = lib.mkOption {
      type = lib.types.str;
      default = "localhost";
    };

    origin = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      example = "https://my.site";
      default = null;
    };

    node-package = lib.mkPackageOption pkgs "node" { };
  };

  config.systemd.services.tableaux-playground = lib.mkIf cfg.enable {
    description = "Countdown timer";
    after = [ "network.target" ];
    wantedBy = [ "multi-user.target" ];

    serviceConfig = commonServiceConfig // {
      ExecStart = "${pkgs.nodejs}/bin/node ${tableaux-playground}/playground/build";
      StateDirectory = "tableaux";
      SyslogIdentifier = "tableaux";
      RuntimeDirectory = "tableaux";
    };

    environment = {
      "PORT" = "${toString cfg.port}";
      "HOST" = cfg.host;
      "ORIGIN" = lib.mkIf (cfg.origin != null) cfg.origin;
    };
  };
}
