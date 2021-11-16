FROM golang:1.17

WORKDIR /go/src
COPY . .

ENV PATH="/go/bin:${PATH}"

RUN GOOS=linux go build -ldflags="-s -w" -o BarrelsServer .

ENTRYPOINT ["./BarrelsServer"]
