CC = clang

all: debug

release: CFLAGS = -O2
release: app

debug: CFLAGS = -Werror -g -Wall
debug: app

llvm: src/main.c
	mkdir -p llvm
	clang -S -emit-llvm -lm src/main.c -o llvm/main.ll
	clang -S -emit-llvm -lm src/parser.c -o llvm/parser.ll
	clang -S -emit-llvm -lm src/eval.c -o llvm/eval.ll
	clang -S -emit-llvm -lm src/reorder.c -o llvm/reorder.ll
	clang -S -emit-llvm -lm src/node_containers.c -o llvm/node_containers.ll

app: setup main.o parser.o eval.o reorder.o node_containers.o
	$(CC) $(CFLAGS) -o plot obj/main.o obj/parser.o obj/eval.o obj/reorder.o obj/node_containers.o -lm

node_containers.o: src/node_containers.c
	$(CC) $(CFLAGS) -c src/node_containers.c -o obj/node_containers.o

reorder.o: src/reorder.c
	$(CC) $(CFLAGS) -c src/reorder.c -o obj/reorder.o

parser.o: src/parser.c
	$(CC) $(CFLAGS) -c src/parser.c -o obj/parser.o

eval.o: src/eval.c
	$(CC) $(CFLAGS) -c src/eval.c -o obj/eval.o

main.o: src/main.c
	$(CC) $(CFLAGS) -c src/main.c -o obj/main.o

setup:
	mkdir -p obj

clean:
	rm -rf llvm
	rm -rf obj
	rm -f plot *.o *.ll
