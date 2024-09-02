#!/bin/bash

makepkg -g >> PKGBUILD
makepkg --printsrcinfo > .SRCINFO
cp PKGBUILD aur_pkg/
cp .SRCINFO aur_pkg/
