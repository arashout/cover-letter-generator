test:
	cargo build && cat test_descriptions/attendease.txt | ./target/debug/cv-generator -m -c="Attendease" -p="Front-End Web Developer" -d