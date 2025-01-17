{ config, pkgs, lib, ... }:

let
  cfg = config.services.post;
in
{
  options.services.post = {
    enable = lib.mkEnableOption "post";
    package = lib.mkPackageOption pkgs "post" { };
    listen = {
      addr = lib.mkOption {
        type = lib.types.str;
        description = "The ip address the http listener should be listening on.";
        default = "::";
      };
      port = lib.mkOption {
        type = lib.types.port;
        description = "The port the http listener should be listening on.";
        default = 9876;
      };
    };
    smtp = {
      addr = lib.mkOption {
        type = lib.types.str;
        description = "The ip address the smtp server is listening on.";
        default = "::1";
      };
      port = lib.mkOption {
        type = lib.types.port;
        description = "The port address the smtp server is listening on.";
        default = 25;
      };
    };
    templateGlob = lib.mkOption {
      type = lib.types.str;
      description = "The glob pattern where email templates can be found.";
    };
    apiTokenFile = lib.mkOption {
      type = lib.types.str;
      description = "The path of the while which contains the api token.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];

    systemd.services.post = {
      description = "post";

      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      environment =
        let
          addrToString = addr: port: "${if (lib.hasInfix ":" addr) then "[${addr}]" else addr}:${toString port}";
        in
        {
          POST_LISTEN_ADDR = addrToString cfg.listen.addr cfg.listen.port;
          POST_SMTP_ADDR = addrToString cfg.smtp.addr cfg.smtp.port;
          POST_TEMPLATE_GLOB = cfg.templateGlob;
          POST_API_TOKEN_FILE = "%d/api_token";
        };

      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        DynamicUser = true;
        User = "post";
        LoadCredential = "api_token:${cfg.apiTokenFile}";
      };
    };
  };
}

