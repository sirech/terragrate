provider "docker" {
  version = "~> 2.7"
}

resource "docker_image" "image" {
  name = "nginx:alpine"
}

module "public_network" {
  source = "./network"
  name   = "public_network"
}

module "private_network" {
  source   = "./network"
  name     = "private_network"
  internal = true
}

resource "docker_container" "public_container" {
  image = docker_image.image.name
  name  = "public_container"

  command    = ["nginx", "-g", "daemon off;"]
  entrypoint = []
  env        = []

  privileged        = false
  publish_all_ports = false

  max_retry_count = 0
  memory          = 0
  memory_swap     = 0
  shm_size        = 64

  networks_advanced {
    name = module.public_network.name
  }
}

resource "docker_container" "private_container" {
  image = docker_image.image.name
  name  = "private_container"

  command    = ["nginx", "-g", "daemon off;"]
  entrypoint = []
  env        = []

  privileged        = false
  publish_all_ports = false

  max_retry_count = 0
  memory          = 0
  memory_swap     = 0
  shm_size        = 64

  networks_advanced {
    name = module.private_network.name
  }
}
