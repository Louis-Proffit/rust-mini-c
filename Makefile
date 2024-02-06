
test:
	@cargo run test.c test.s
	@gcc -s test.s -o test.o
	@./test.o
	@echo $$?

clean:
	rm test.s
	rm test.o