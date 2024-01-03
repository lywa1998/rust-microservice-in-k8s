= Communication
Gateway <-> Service 1 <-> service 2
```proto
syntax = "proto3";

package api;

service EchoService {
  rpc EchoFromService1 (EchoRequest) returns (EchoResponse);
  rpc EchoFromService2 (EchoRequest) returns (EchoResponse);
}

message EchoRequest {
  string message = 1;
}

message EchoResponse {
  string message = 1;
}
``` 

= Service

== Gateway 
The gateway service distribute http request to service 1.

= Dockerize microservice application
docker build #footnote[https://github.com/keithsharp/cloud-native-rust]
When use mac appision to make a cross-compilation, you may be see a error: `unrecongize a -m64`.
Try to add `ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc` to your Dockerfile#footnote[https://stackoverflow.com/questions/69360099/apple-m1-to-linux-x86-64-unrecognized-command-line-option-m64].