{config, lib, pkgs, ... }:
let
    # resources used to make
    # https://nixos.wiki/wiki/Extend_NixOS
    inherit (lib) mkIf mkOption mkEnableOption optionalAttrs types;
    cfg = config.services.sunServer;
in{
    options = {
        services.sunServer = {
            enable = mkEnableOption "Enables sunServer";
            package = mkOption {
              type = types.package;
              defaultText = literalExpression "inputs.sun_server.packages.x86_64-linux.default";
              example = literalExpression "inputs.sun_server.packages.x86_64-linux.default";
              description = lib.mdDoc ''
                A server to serve lighting curves to Home Assistant.
              '';
            };
            user = mkOption { #TODO look at upstream for example
                type = with types; str;
                default = "sunserver";
            };
            dataDir = mkOption {
              type = types.str;
              default = "/var/lib/sunserver";
              description = lib.mdDoc ''
              Path to the sunserver data directory
              '';
            };
            port = mkOption { #TODO look at upstream for example
                type = with types; port;
                default = 3000;
            };
        };
    };
    config = mkIf cfg.enable {
        networking = {
          firewall = {
            # allowedTCPPorts = [];
            allowedUDPPorts = [cfg.port];
          };
        };

        users.users.${cfg.user} =
            optionalAttrs (cfg.user == "sunserver") {
              isSystemUser = true;
            #   group = cfg.group;
              home = cfg.dataDir;
              createHome = true;
            };

        systemd.services.sunServer = {
          # this service is "wanted by" (see systemd man pages, or other tutorials) the system 
          # level that allows multiple users to login and interact with the machine non-graphically 
          # (see the Red Hat tutorial or Arch Linux Wiki for more information on what each target means) 
          # this is the "node" in the systemd dependency graph that will run the service
          wantedBy = [ "multi-user.target" ];
          # systemd service unit declarations involve specifying dependencies and order of execution
          # of systemd nodes; here we are saying that we want our service to start after the network has 
          # set up (as our IRC client needs to relay over the network)
          after = [ "network.target" ];
          description = "A server to send lighting curves to Home Assistant.";
          serviceConfig = {
            # see systemd man pages for more information on the various options for "Type": "notify"
            # specifies that this is a service that waits for notification from its predecessor (declared in
            # `after=`) before starting
            Type = "simple";
            # username that systemd will look for; if it exists, it will start a service associated with that user
            User = cfg.user;
            PrivateTmp = "yes";

            # the command to execute when the service starts up 
            ExecStart = ''${cfg.package}/bin/sun_server''; 
            # and the command to execute         
            # ExecStop = ''${pkgs.screen}/bin/screen -S irc -X quit'';
          };
          preStart = ''
            mkdir -p ${cfg.dataDir}
            chown ${cfg.user} ${cfg.dataDir}
            '';
        };
        
    };
}