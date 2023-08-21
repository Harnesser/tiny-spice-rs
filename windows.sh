#! /usr/bin/env sh

PRJDIR=/home/${USER}/Projects/tiny-spice-rs

cd $PRJDIR
 
gvim \
	-geometry 100x100+900+10 \
	-S \
	&


gnome-terminal \
	--geometry=100x100+10+10 \
	--tab-with-profile=DEFAULT \
		--title="tiny-spice-rs" \
		--working-directory=${PRJDIR} &


