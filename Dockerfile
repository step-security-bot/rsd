FROM cgr.dev/chainguard/rust@sha256:723589969dd57c335f09d46f5a07590ce2a3f8e1ff58271eb8e33788eea30295 as build

WORKDIR /build

COPY . .

RUN make build

FROM cgr.dev/chainguard/static@sha256:68b8855b2ce85b1c649c0e6c69f93c214f4db75359e4fd07b1df951a4e2b0140

COPY --from=build /build/rsd /rsd

ENTRYPOINT ["/rsd"]
