default: build

%.svg: %.puml
	plantuml $< -tsvg

install:
	brew install plantuml


build: \
  1.create-root-identity.svg \
  2.authenticate-with-platform.svg \
  3.authenticate-with-service-direct.svg \
  4.authenticate-with-service-via-platform.svg
