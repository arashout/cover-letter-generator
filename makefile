test:
	cargo build && cat test_descriptions/attendease.txt | ./target/debug/cv-generator -m -c="Attendease" -p="Front-End Web Developer" -d
jobbot:
	cargo build --release && cp ./target/debug/cv-generator ../JobBot/cv-generator