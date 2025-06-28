<p align="center">
    <img src=".github/assets/header.png" alt="Uzinfocom's {Jobful}">
</p>

<p align="center">
    <h3 align="center">Uzinfocom's Job & Vacancy micromanagement telegram bot.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/uzinfocom-org/jobful?style=flat&logo=rust&logoColor=ffffff&labelColor=242424&color=242424" alt="Top Used Language">
    <a href="https://github.com/uzinfocom-org/jobful/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/uzinfocom-org/jobful/test.yml?style=flat&logo=github&logoColor=ffffff&labelColor=242424&color=242424" alt="Test CI"></a>
</p>

## About

This is a telegram bot bootstrapped with [Bleur Stack] and written on Rust & Nix that helps our HRs to publish and announce vacancy posters and announcements throughout Floss Network. It was craeted in order to feed job-hungry and talented youth with possibilities.

## Features

- Automatic announcement management
- Details about our HR and further vacancy listings.

## Development

The project has `shell.nix` which has development environment preconfigured already for you. Just open your
terminal and at the root of this project:

```bash
# Open in bash by default
nix develop

# If you want other shell
nix develop -c $SHELL

# Upon entering development environment for the first
# time, you'll be asked for your development telegram
# bot token, it will be written to .env file for more
# convenient dev env startups. Token is saved at .env
# file at the root of this project. You can change it
# whenever you want!

# After entering development environment, inside the
# env, you can open your editor, so your editor will
# read all $PATH and environmental variables, also
# your terminal inside your editor will adopt all
# variables, so, you can close terminal.

# Neovim
vim .

# VSCode
code .

# Zed Editor
zed .
```

The development environment has whatever you may need already, but feel free to add or remove whatever
inside `shell.nix`.

## Building

Well, there are two ways of building your project. You can either go with classic `cargo build` way, but before that, make sure to enter development environment to have cargo and all rust toolchain available in your PATH, you may do like that:

```bash
# Entering development environment
nix develop -c $SHELL

# Compile the project
cargo build --release
```

Or, you can build your project via nix which will do all the dirty work for you. Just, in your terminal:

```bash
# Build in nix environment
nix build

# Executable binary is available at:
./result/bin/jobful
```

## Deploying (works only for flake based NixOS)

Deploying this project, telegram bot requires host machine to have its own flake based configuration.

### Activation

In your configuration, add your project repository to `inputs`.

```nix
{
  inputs = {
    # ...

    # Let's imagine name of this project as `tempbot`
    jobful.url = "github:uzinfocom-org/jobful";
  };
}
```

Ok, now we have your project in repository list and now, we need to make use of options provided by modules of your project. In order to do that, we need to activate our module by importing our module. In your configuration.nix, find where you imported things and then add your project like that:

```nix
# Most of the time it's at the top part of nix configurations
# and written only once in a nix file.
{ ... }: {
  # ... something

  # And here begins like that
  imports = [
    # Imagine here your existing imports

    # Now import your project module like this
    inputs.jobful.nixosModules.bot
  ];
};
```

Alright! Since we imported the module of our project and options are now available, now head into setting up section!

### Set up

Options are available, modules are activated and everything is ready to deploy, but now, we need to explain NixOS how
to deploy our project by writing some Nix configs. I already wrote some options and configurations which will be available
by default after project bootstrap, you are free to modify, add and remove whatever inside `module.nix` to your
liking. If you need list of available default options or explanations for every option, refer to [available default options] section below. In this guide, I'll
be showing you an example set up you may use to get started very fast, you'll find out the rest option by yourself if you
need something else. In your `configuration.nix` or wherever of your configuration:

```nix
{
  services.jobful-bot = {
    # Enable systemd service
    enable = true;

    # Telegram bot token passed to your bot via arguments
    token = "/srv/bot-token";

    # Enabling webhook integration which activates
    # caddy or nixos part of nix configuration at
    # `module.nix`
    webhook = {
      # Activate webhook part of nix configuration
      enable = true;

      # From given options (caddy or nginx), choose
      # web server to deploy bot via an http server
      proxy = "nginx";

      # Domain to pass to web server (caddy or nginx)
      domain = "jobful.something.uz";

      # Port to host http server and tell web proxy
      # to were bind that proxy
      port = 8445;
    };
  };
}
```

This is very basic and minimal example, you can tune other things like user who's going to run this systemd service, change group of user and many more. You can add your own modifications and add more options by yourself.

### Available default options

These are options that are available by default, just put services."jobful-bot" before the keys:

#### `enable` (required) -> bool

Turn on systemd service of telegram bot project.

#### `token` (required) -> path to file

Telegram bot token to pass to telegram bot, it should be a file that can be placed almost anywhere. Inside the file, there should be only telegram bot token as whole content. Don't type telegram bot token directly as value for this option, it was done like that to don't expose your token openly in your public repository or expose it at /nix/store. Also, you can chain it with secret manager like `sops-nix` like that:

```nix
{
  sops.secrets = {
    "mytoken" = {
      owner = config.services.tempbot-bot.user;
    };
  };

  services.tempbot-bot.token = config.sops.secrets."mytoken".path;
}
```

#### `webhook.enable` (optional) -> bool

Enable automatic web proxy configuration for either caddy or nginx. If the value is false, telegram bot will be deployed in `polling` mode. This is for people who have or want complex web server configurations.

#### `webhook.proxy` (optional) -> `caddy` or `nginx` as value

Choose which web server software should be integrated with.

#### `webhook.domain` (optional) -> string

It will be passed to web proxy to let it know whether to which domain the configurations should be appointed to.

#### `webhook.port` (optional) -> integer

Which port should be used to host bot and proxy.

#### `user` (optional) -> string

The user that will run the telegram bot. It's defaulted to "{package.name}-bot".

#### `group` (optional) -> string

Name of a group to which the user that's going to run the telegram bot should be added to. It's defaulted to the name of the user.

#### `dataDir` (optional) -> path

A location where working directory should be set to before starting telegram bot. If you have a code to write something in current working directory, the value to this option is where it will be written. It's defaulted to "/var/lib/{package.name}-bot".

#### `package` (optional) -> nix package

The packaged telegram bot with pre-compiled binaries and whatever. Defaulted to current project's build output and highly suggested to not change value of this option unless you know what you're doing.

## Working productions

This telegram bot is currently deployed at Kolyma's Datacenter and Infrastructure and powered by Uzinfocom's Open Source Team.

## FAQ

### Why not use default.nix for devShell?

There's been cases when I wanted to reproduce totally different behaviors in development environment and
production build. This occurs quite a lot lately for some reason and because of that, I tend to keep
both shell.nix and default.nix to don't mix things up.

## Thanks

- [Template](https://github.com/bleur-org) - Started with this template
- [Orzklv](https://github.com/orzklv) - For making this happen

## License

This project is licensed under the MIT License - see the [LICENSE](license) file for details.

<p align="center">
    <img src=".github/assets/footer.png" alt="Uzinfocom's {Jobdul}">
</p>

[Bleur Stack]: https://github.com/bleur-org
[available default options]: #available-default-options
