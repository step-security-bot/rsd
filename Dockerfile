FROM cgr.dev/chainguard/rust as build

WORKDIR /build

COPY . .

RUN make build

FROM cgr.dev/chainguard/static

COPY --from=build /build/rsd /rsd

ENTRYPOINT ["/rsd"]
