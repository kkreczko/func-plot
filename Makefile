CC = clang

all: debug

release: CFLAGS = -O2
release: app

debug: CFLAGS = -Werror -g -Wall
debug: app

llvm: main.c
	clang -S -emit-llvm -lm main.c -o main.ll
	clang -S -emit-llvm -lm parser.c -o parser.ll
	clang -S -emit-llvm -lm eval.c -o eval.ll

app: main.o parser.o eval.o
	$(CC) $(CFLAGS) -o plot main.o parser.o -lm

parser.o: parser.c
	$(CC) $(CFLAGS) -c parser.c

eval.o: eval.c
	$(CC) $(CFLAGS) -c eval.c

main.o: main.c
	$(CC) $(CFLAGS) -c main.c

clean:
	rm -f plot *.o *.ll
