FROM docker.io/qcidocker/qci_base:latest

RUN wget https://dl.google.com/go/go1.12.4.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go1.12.4.linux-amd64.tar.gz        && \
    rm go1.12.4.linux-amd64.tar.gz                            && \
    export PATH=/usr/local/go/bin:$PATH                       && \
    go version

ENV PATH=/usr/local/go/bin:$PATH
