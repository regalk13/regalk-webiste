-- title: Why I love Nix
-- desc: Nix changed the way I see containers, pkg managers and the communities. Nix is the modern way to understand systems.
-- image: nixblog.webp
-- date: 2025-02-02

## NixOS: A Love Letter to the Future of Systems

> Let’s be real: modern computing is a dependency hellscape. Ever tried compiling a "complex" piece of software from scratch? Even if you somehow track down every single dependency—and their correct versions, and their configurations—chances are, you’ll still hit a wall. Where are these big projects that used to compile with a command?

Enter NixOS. After months of using it, I’m convinced it’s the answer. Nix (the pkg manager) alone is a game-changer. You don’t need to beg maintainers for specific versions or worry about dependency conflicts nuking your system. Everything is isolated, reproducible, and yours.

I know it don't fix the full problem I mentioned by itself but give tools and insights... But the benefits of use Nix/NixOS still there.


"Nix is a tool that takes a unique approach to package management and system configuration" and NixOs a Linux distro built above this paradigm.

https://nixos.org/

nixblog.webp


## The Flakes

Nix is a declarative package manager that enables users to declare the desired system state in configuration files (declarative configuration), and it takes responsibility for achieving that state.

The flakes experimental feature is a major development for Nix, it introduces a policy for managing dependencies between Nix expressions, it improves reproducibility, composability and usability in the Nix ecosystem. Although it's still an experimental feature, flakes have been widely used by the Nix community. Similar to your package.json, you have your flake.nix. Your package-lock.json flake.lock.
As the nix paradigm defines, everything is self-contained. So you can define your entire system, a development environment, or even a production deployment using flakes, and anyone with Nix can spin it up instantly—no "it works on my machine" nonsense. (most of the time it works in this way).

On the other hand, Flakes experimental features did not break Nix's original design at the user level. The two new files flake.nix/flake.lock introduced by Flakes are just a wrapper for other Nix configurations

The benefits of Flakes are evident, and the entire NixOS community has embraced it wholeheartedly. Currently, more than half of the users utilize Flakes, providing assurance that Flakes will not be deprecated.

Want to replicate your setup on another machine? Clone the flake and run nixos-rebuild. Done.

https://nixos-and-flakes.thiscute.world/preface

## The system

I like the way of threat services on config, services.whatever.enable = true; and watch it materialize. It still atomic!

The future of systems seems to be atomic, new distros become with similar paradigm (guys did you see the SteamOs atomic system??) So why not learn it faster as it gets? See systems as projects and projects as systems runs everywhere works everywhere

The magic is in the declarative config too, even just do a roll-back. Your system is bullet proof.


Reproducibility.

Isolation.

Flexibility.

Community.

Peace.

## Take Aways

Nix compiler can be slow, and no expect any descriptive error message sometimes. (This is not that bad...)

Making cool stuff with it takes time, when you get there its beautiful. You still need time to get your flake working correctly. You still need docker or podman sometimes, but it replaces a lot of stuff by itself.

Generations and caches eat disk space. !! nix-collect-garbage

Nix have a really bright future, you will see more companies usign it for convenience, google with idx as an example. I'm still learning more from it and I want to create more cool stuff with it. (This page use nix btw)

Cool projects:

https://nixos.org/guides/nix-pills/

https://lix.systems/about/

https://nixos-and-flakes.thiscute.world/preface
