SHELL := /usr/bin/env bash

PREFIX ?= /usr
DESTDIR ?=
SHAREDIR := $(DESTDIR)$(PREFIX)/share/hyprland-crt-shader
BINDIR := $(DESTDIR)$(PREFIX)/bin
LICENSEDIR := $(DESTDIR)$(PREFIX)/share/licenses/hyprland-crt-shader

.PHONY: all check install uninstall package clean

all: check

check:
	bash -n bin/hypr-crt-toggle
	python -m py_compile bin/hypr-crt-control
	rm -rf bin/__pycache__
	@if command -v glslangValidator >/dev/null 2>&1; then \
		glslangValidator -S frag shaders/crt.frag; \
	else \
		echo "warning: glslangValidator not found; skipping GLSL validation"; \
	fi

install:
	install -Dm644 shaders/crt.frag "$(SHAREDIR)/crt.frag"
	install -Dm755 bin/hypr-crt-toggle "$(BINDIR)/hypr-crt-toggle"
	install -Dm755 bin/hypr-crt-control "$(BINDIR)/hypr-crt-control"
	install -Dm644 config/hyprland-crt-shader.lua "$(SHAREDIR)/hyprland-crt-shader.lua"
	install -Dm644 config/hyprland-crt-shader.conf "$(SHAREDIR)/hyprland-crt-shader.conf"
	install -Dm644 LICENSE "$(LICENSEDIR)/LICENSE"

uninstall:
	rm -f "$(BINDIR)/hypr-crt-toggle" "$(BINDIR)/hypr-crt-control"
	rm -rf "$(SHAREDIR)" "$(LICENSEDIR)"

package:
	./scripts/build-arch-package.sh

clean:
	rm -rf build dist/*.pkg.tar.*
