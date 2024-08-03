{ config, pkgs, lib, ... }:

let
  cfg = config.services.post;
in
{
  options.services.post = {
    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.post;
      defaultText = lib.literalExpression "pkgs.post";
      description = "Which post derivation to use.";
    };
    enable = lib.mkEnableOption "post";
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
          listenAddr = cfg.listen.addr;
          smtpAddr = cfg.listen.addr;
        in
        {
          POST_LISTEN_ADDR = "${if (lib.hasInfix ":" listenAddr) then "[${listenAddr}]" else listenAddr}:${toString cfg.listen.port}";
          POST_SMTP_ADDR = "${if (lib.hasInfix ":" smtpAddr) then "[${smtpAddr}]" else smtpAddr}:${toString cfg.smtp.port}";
          POST_TEMPLATE_GLOB = cfg.templateGlob;
          POST_API_TOKEN_FILE = cfg.apiTokenFile;
        };

      serviceConfig = {
        ExecStart = "${cfg.package}/bin/post";
        DynamicUser = true;
        User = "post";
      };
    };
  };
}

