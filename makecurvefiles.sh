for i in {1..50}; do
	./target/debug/randomcurve --svg --closed > temp.svg
	openscad test.scad -o ./out/curve$i.stl
done
