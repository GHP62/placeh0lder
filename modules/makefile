CC=x86_64-w64-mingw32-gcc -O0
INC=./include

all: clean build

clean:
	rm -fr ./build

build:
	mkdir ./build
	${CC} -o ./build/placeholding ./src/main.c ./src/fullRemove.c -I ${INC}
	${CC} -o ./build/main ./src/modules/*.c ./src/fullRemove.c -I ${INC}
