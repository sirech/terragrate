# terragrate

[![Crate version](https://img.shields.io/crates/v/terragrate)](https://crates.io/crates/terragrate) ![linux-release](https://github.com/sirech/terragrate/workflows/linux-release/badge.svg) ![macos-release](https://github.com/sirech/terragrate/workflows/macos-release/badge.svg) ![License](https://img.shields.io/crates/l/terragrate)

_terragrate_ helps doing state migrations in [terraform](https://www.terraform.io/). It is inspired by database migration tools like [Flyway](https://flywaydb.org/).

## Getting started

### Prerequisites

You'll need to install `terraform` and set it up. At least `terraform state list` should work for `terragrate` to be any useful.

### Installation

Download the [latest release](https://github.com/sirech/terragrate/releases) for your OS and put it in your `PATH`. Running `terragrate` alone will print the detailed help.

## Usage

### Why `terragrate`?

Let's assume I have a resource like this in `terraform`:

```hcl
resource "docker_network" "network" {
  name     = "public_network"
}
```

I have provisioned the resource, which is identified as `public_network.docker_network.network` in the state.

During a refactoring, we extract this resource to a module called _network_ to make it more reusable. Now we will use it a bit differently: 

```hcl
module "public_network" {
  source = "./network"
  name   = "public_network"
}
```

If we run `terraform apply`, it will try to destroy the old resource and create a new one. Even though it's the exact same entity, the new id is called `module.public_network.docker_network.network`.

This is fine for a toy example like this, but it makes no sense to reprovision a complete database just because of this!

The typical approach to fix this is by migrating the state with the [terraform state commands](https://www.terraform.io/docs/commands/state/index.html) by hand. This is **error-prone** and **untraceable**.

### Using `terragrate`

Instead, I propose using a migration reflected in code. Let's define the `move_to_module.json` migration, and store it together with the infrastructure code:

```json
{
  "name": "move to module",
  "description": "Convert network to module",
  "transformations": [
    {
      "kind": "MV",
      "matcher": "public_network",
      "replacement": "module.public_network"
    }
  ]
}
```

I can use that migration and feed it my current state with this:

```console
terraform state list | terragrate --state - --migration move_to_module.json tf
```

This will output the list of `terraform` commands needed to migrate the state properly, in this case:

```console
terraform state mv module.public_network.docker_network.network module.module.public_network.docker_network.network
```

By keeping the migration in your repository, you can known which migrations have been applied thus far. An automated script is much less likely to leave your state in an undefined situation.

#### Transformation types

The following transformations are supported:

- _MV_: Move items in [terraform state](https://www.terraform.io/docs/commands/state/mv.html).
- _RM_: /remove items from the [terraform state](https://www.terraform.io/docs/commands/state/mv.html).

## Current limitations

- As of today, `terragrate` does not offer a way of automatically running migrations in case your state is out of date.
- Right now, `terragrate` assumes that you call `terraform` directly. It doesn't support something like [terragrunt](https://terragrunt.gruntwork.io/)

## License

See [LICENSE](./LICENSE)

## Roadmap

See the [open issues](https://github.com/sirech/terragrate/issues) for a list of proposed features (and known issues).
