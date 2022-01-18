default: build

%.svg: %.puml
	plantuml $< -tsvg

install:
	brew install plantuml


build: \
  sequence_diagrams/examples/1.create-root-identity.svg \
  sequence_diagrams/examples/2.authenticate-with-platform.svg \
  sequence_diagrams/examples/3.authenticate-with-service-direct.svg \
  sequence_diagrams/examples/4.authenticate-with-service-via-platform.svg \
  sequence_diagrams/examples/5.create-intermediate-identity.svg \
  sequence_diagrams/examples/6.revoke-intermediate-identity.svg
