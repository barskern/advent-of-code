src/day%.rs: mal.rs.tmpl
	sed 's/{day}/day$*/g' $^ > $@
	sed -i '1s/^/pub mod day$*;\n/' src/lib.rs
