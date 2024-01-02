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
