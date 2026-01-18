.DEFAULT_GOAL := build

.PHONY:fmt vet build clean

build:
		cargo build