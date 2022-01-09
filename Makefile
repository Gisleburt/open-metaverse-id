default: build

%.svg: %.puml
	plantuml $< -tsvg

install:
	brew install plantuml


build: 1.create-root-identity.svg 2.authenticate-with-platform.svg
