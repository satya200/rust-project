CC=gcc
CFLAGS=-fPIC -Wall -Werror
TARGET=libdeviceutils-lib.so

all: $(TARGET)

$(TARGET): deviceutils.o
	$(CC) -shared -o $@ $^

deviceutils.o: deviceutils.c deviceutils.h
	$(CC) $(CFLAGS) -c deviceutils.c

clean:
	rm -f *.o *.so

