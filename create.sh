for i in {1..25}; do
	cargo new day_$i;
	cd day_$i/src;
	rm main.rs;
	touch input.txt;
	cp ../../template.rs ./main.rs;
	cd ../../;
done
