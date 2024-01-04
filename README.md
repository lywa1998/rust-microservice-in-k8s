# Rust-Microservice-K8S

write a microservice project, and deploy services to k8s.

## Usage
Router Tree
```
localhost:8000!
├──[GET] -> gateway::hello
└──echo
    └──[POST] -> gateway::echo
```

Test
```
curl -X GET localhost:8000
// Hello, this a echo microservice project.
```

```
curl -X POST -d 'message=ABC' localhost:8000/echo
// Hello ABC from gateway from service 2!
// This message is forwarded from service 1!
```

## Resourse
- [Rust language-specific guide from Docker website](https://docs.docker.com/language/rust/)
- [Rust and Dockerfile examples](https://github.com/keithsharp/cloud-native-rust)

## TODO
- [x] clean unused crate
- [x] microservice code
- [x] docker build
- [x] docker compose
- [x] k8s environment
- [x] deploying microservice in k8s
- [ ] CI/CD
- [ ] write doc

## Problem
- Cargo-chef pre-build binary is build in amd64, and is not compile with apple silicon machine.