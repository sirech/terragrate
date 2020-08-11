# terragrate

[![Crate version](https://img.shields.io/crates/v/terragrate)](https://crates.io/crates/terragrate) ![linux-release](https://github.com/sirech/terragrate/workflows/linux-release/badge.svg) ![macos-release](https://github.com/sirech/terragrate/workflows/macos-release/badge.svg) ![License](https://img.shields.io/crates/l/terragrate)

## What's the purpose of _terragrate_?

_terragrate_ aims to help doing state migrations in [terraform](https://www.terraform.io/), inspired by database migration tools like [Flyway](https://flywaydb.org/).

## Usage

Let's assume I have a resource like this:

```hcl
resource "docker_network" "network" {
  name     = "public_network"
}
```

I have provisioned it as part my infrastructure, and the resource is called now `public_network.docker_network.network`.

During a refactoring, we extract this resource to a module called 'network' to make it more reusable. Now we will use it by writing: 

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

Instead, I propose using a migration reflected in code. Let's define the `move_to_module.json` migration:

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

## Current limitations

- As of today, `terragrate` does not offer a way of automatically running migrations in case your state is out of date.
- Right now, `terragrate` assumes that you call `terraform` directly. It doesn't support something like [terragrunt](https://terragrunt.gruntwork.io/)

## Pending

- `import` command
- Configure `terraform` command
- Upload sha1
- publish to crates.io

