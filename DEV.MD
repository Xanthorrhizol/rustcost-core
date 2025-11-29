

Developer notes, build instructions, scripts


```bash
cd C:\code\rustcost\rustcost-core
docker build --build-arg APP_NAME=rustcost-core -t rustcost-core .
docker tag rustcost-core kimc1992/rustcost-core:1.0.0-dev.17
docker push kimc1992/rustcost-core:1.0.0-dev.17
```