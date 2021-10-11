FROM ubuntu:20.04
USER root

WORKDIR /srv/src/api

RUN apt-get -y update \
 && DEBIAN_FRONTEND=noninteractive apt-get -yq install curl apt-utils make \
 && curl -sL https://deb.nodesource.com/setup_14.x | bash - \
 && apt-get install -y nodejs

RUN echo sup

COPY . /srv/src/api

RUN cd /srv/src/api && npm -g config set user root && sudo npm install -g --include dev
