<img src="https://api.iconify.design/material-symbols/deployed-code-update-outline.svg?color=%23D2E0F0" width="100px" align="left">

### `Vibrant`

![Latest tag](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fcodeberg.org%2Fapi%2Fv1%2Frepos%2FHeartblin%2FVibrant%2Ftags%3Flimit%3D1&query=%24%5B0%5D.name&label=latest%20tag)

<br />
A small wrapper/helper for nix rebuilds.

### Disclaimer

> [!CAUTION]
> This repo is to be considered unstable.
> Nothing here is going to be ever finalised.
>
> This is mostly a project for me to experiment with Rust.
> This tool makes assumptions about the system and environment.
> Do not consider this stable, or worse: production-grade.
> While I do daily drive this, your mileage may vary.
>
> [There are way better alternatives to this.](###special-thanks)

### Installation

For NixOS, just add it as an input:

```nix
{
  inputs.vibrant = {
    url = "git+https://codeberg.org/HeartBlin/Vibrant.git";
    inputs.nixpkgs.follows = "nixpkgs"; # Optionally follow `nixpkgs`
  };
}
```

And import it in a file that a host imports:

```nix
{ inputs, ... }: { imports = [ inputs.vibrant.nixosModules.default ]; }
```

After that, `programs.vibrant.*` becomes avalible for use:

```nix
{
  programs.vibrant = {
    enable = true;
    flake = "/absolute/path/to/flake"; # Sets 'VIBRANT_DIR'
  };
}
```

### Usage

In its current form, `Vibrant` is quite feature-less, it being only in charge of rebuilds. It figures out what `nixosConfiguration` to build from the hosts `/etc/hostname` file. That means your system's `attrname` must match its `hostname`.

The flake location is automatically aquired from a set environment variable, namely: `VIBRANT_DIR`, which the module can set.

```bash
λ vibrant --help
Usage: vibrant [OPTIONS]

Options:
  -u, --update   Update all flake inputs
  -h, --help     Print help
  -V, --version  Print version
```

* `--update / -u`: Does a `nix flake update` (actually using [`nom`](###special-thanks)) before a build.

### Roadmap

I intend to keep the tools scope contrived, so most of these will be about easing my work with `nixox-rebuild`, rather than convering all of its edge-cases.

Still, there are some extra features I want to work on:

- [x] ~~_Flake update functionality._~~
- [ ] Choosing between `boot` or `switch`.
- [ ] Hostname overriding.
- [ ] Remote building.
- [ ] A nicer CLI (my own prints).

### Special Thanks
In no particular order:

* [`nh`](https://github.com/nix-community/nh): Serving as inspiration, despite this project being a mere wrapper. Use this instead of `Vibrant`, it just works better.
* [`nom`](https://github.com/maralorn/nix-output-monitor): What powers most of this tools pretty graphics. In charge of showing the pretty graphs while building.
* [`nvd`](https://sr.ht/~khumba/nvd/): Powers the diffs after a build succedes.
* And to the broader Rust community, which drove me to try the language out in the first place

---

Licensed under the [European Union Public Licence v1.2 (EUPL-1.2)](LICENSE).
